using System;
using System.Linq;
using System.Collections.Generic;
using System.Collections.Immutable;

using ArenaGS.Utilities;
using ArenaGS.Engine;
using ArenaGS.Platform;

namespace ArenaGS.Model
{
	internal struct GeneratedMapData
	{
		internal Map Map { get; }
		internal ImmutableList<MapScript> Scripts { get; }

		internal GeneratedMapData (Map map, ImmutableList<MapScript> scripts)
		{
			Map = map;
			Scripts = scripts;
		}
	}

	internal interface IMapGenerator
	{
		GeneratedMapData Generate (int hash);

		// Scripts should be serialized seperate to disk to keep state
		Map Regenerate (int hash);
	}

	internal class SimpleMapGenerator : IMapGenerator
	{
		IGenerator Generator;
		public SimpleMapGenerator ()
		{
			Generator = Dependencies.Get<IGenerator> ();
		}

		public GeneratedMapData Generate (int hash)
		{
			int width = 25;
			int height = 20;
			Map map = new Map (width, height, "Simple", MapTheme.Mud, hash, 42);
			for (int i = 1; i < width - 1; ++i)
				for (int j = 1; j < height - 1; ++j)
					map.Set (new Point (i, j), TerrainType.Floor);


			List<MapScript> scripts = new List<MapScript> ();

			scripts.Add (Generator.CreateSpawner (new Point (18, 8)));

			return new GeneratedMapData (map, scripts.ToImmutableList ());
		}

		public Map Regenerate (int hash)
		{
			return Generate (hash).Map;
		}
	}

	class OpenArenaMapGenerator : IMapGenerator
	{
		const int MinDimensionSize = 14;
		const int MaxDimensionSize = 24;
		public GeneratedMapData Generate (int hash)
		{
			Random rng = new Random(hash);
			Map map = GenerateMap (hash, rng);
			return new GeneratedMapData(map, new MapScript[] { }.ToImmutableList());
		}
		
		enum MapShape
		{
			Rectangle,
			Square,
			Circle
		}

		enum MapAdditions
		{
			None,
			Center,
			CenterSpread,
			Spread
		}

		void DigRectangleCenter (Map map, int width, int height)
		{
			for (int i = 1; i < width - 1; ++i)
				for (int j = 1; j < height - 1; ++j)
					map.Set (new Point (i, j), TerrainType.Floor);
		}

		void DigCircleCenter (Map map, int size)
		{
			Point center = new Point (size / 2, size / 2);
			foreach (Point p in center.PointsInBurst ((size / 2) - 1))
				map.Set (p, TerrainType.Floor);
		}

		Map GenerateMap (int hash, Random rng)
		{
			Map map;
			MapShape mapShape = (MapShape)rng.Next (3);
			MapTheme mapTheme = (MapTheme)rng.Next (Enum.GetValues(typeof(MapTheme)).Length);

			switch (mapShape)
			{
				case MapShape.Rectangle:
				{
					int width = rng.Next (MinDimensionSize, MaxDimensionSize).MakeOdd ();
					int height = rng.Next (MinDimensionSize, MaxDimensionSize).MakeOdd ();
					map = new Map (width, height, "OpenArenaMap", mapTheme, hash, rng.Next());
					DigRectangleCenter (map, width, height);
					break;
				}
				case MapShape.Square:
				{
					int width = rng.Next (MinDimensionSize, MaxDimensionSize).MakeOdd ();
					int height = width;
					map = new Map (width, height, "OpenArenaMap", mapTheme, hash, rng.Next());
					DigRectangleCenter (map, width, height); 
					break;
				}
				case MapShape.Circle:
				{
					int size = rng.Next (MinDimensionSize, MaxDimensionSize).MakeOdd ();
					map = new Map (size, size, "OpenArenaMap", mapTheme, hash, rng.Next());
					DigCircleCenter (map, size);
					break;
				}
				default:
					throw new NotImplementedException ();
			}

			MapAdditions additionType = (MapAdditions)rng.Next (4);
			if (additionType == MapAdditions.Spread && mapShape == MapShape.Circle)
				additionType = MapAdditions.None;

			switch (additionType)
			{
				case MapAdditions.None:
					break;
				case MapAdditions.Center:
				{
					int centerWidth = rng.CoinFlip () ? 1 : 3;
					int centerHeight = rng.CoinFlip () ? 1 : 3;
					int [] offset = { 0, 1, -1};

					for (int i = 0 ; i < centerWidth ; ++i)
						for (int j = 0 ; j < centerHeight ; ++j)
							map.Set (new Point ((map.Width / 2) + offset[i], (map.Height / 2) + offset[j]), TerrainType.Decoration);
					map.Set (new Point ((map.Width / 2), (map.Height / 2)), TerrainType.DecorationSpecial);
					break;
				}
				case MapAdditions.CenterSpread:
				{
					int centerWidth = rng.CoinFlip () ? 1 : 3;
					int centerHeight = rng.CoinFlip () ? 1 : 3;
					int [] offset = { 0, 1, -1};

					for (int i = 0 ; i < centerWidth ; ++i)
						for (int j = 0 ; j < centerHeight ; ++j)
							map.Set (new Point ((map.Width / 2) + offset[i], (map.Height / 2) + offset[j]), TerrainType.Decoration);
					map.Set (new Point ((map.Width / 2), (map.Height / 2)), TerrainType.DecorationSpecial);

					int left = (map.Width / 2) - ((centerWidth - 1) / 2);
					int right = (map.Width / 2) + ((centerWidth - 1) / 2);
					int top = (map.Height / 2) - ((centerHeight - 1) / 2);
					int bottom = (map.Height / 2) + ((centerHeight - 1) / 2);

					map.Set (new Point (left - 1, top - 1), TerrainType.Decoration);
					map.Set (new Point (right + 1, top - 1), TerrainType.Decoration);
					map.Set (new Point (left - 1, bottom + 1), TerrainType.Decoration);
					map.Set (new Point (right + 1, bottom + 1), TerrainType.Decoration);
					break;
				}
				case MapAdditions.Spread:
				{
					int groupsWide = rng.CoinFlip () ? 3 : 5;
					int groupsHigh = rng.CoinFlip () ? 3 : 5;
					int distantApartWidth = map.Width / groupsWide;
					int distantApartHeight = map.Height / groupsHigh;

					// The first and last point are the walls
					for (int i = 1 ; i < groupsWide ; ++i)
						for (int j = 1 ; j < groupsHigh ; ++j)
							map.Set (new Point (distantApartWidth * i, distantApartHeight * j), TerrainType.DecorationSpecial);

					break;
				}
			}

			return map;
		}

		public Map Regenerate (int hash)
		{
			return GenerateMap (hash, new Random (hash));
		}
	}
}
