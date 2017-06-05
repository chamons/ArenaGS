using System;
using System.Linq;
using System.Collections.Generic;
using System.Collections.Immutable;

using ArenaGS.Utilities;
using ArenaGS.Engine;

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
			int width = 15;
			int height = 10;
			Map map = new Map (width, height, "Simple", hash);
			for (int i = 1; i < width - 1; ++i)
				for (int j = 1; j < height - 1; ++j)
					map.Set (new Point (i, j), TerrainType.Floor);


			List<MapScript> scripts = new List<MapScript> ();

			scripts.Add (Generator.CreateSpawner (new Point (8, 8)));

			return new GeneratedMapData (map, scripts.ToImmutableList ());
		}

		public Map Regenerate (int hash)
		{
			return Generate (hash).Map;
		}
	}
}
