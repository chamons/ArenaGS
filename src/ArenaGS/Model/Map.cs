using System.Collections.Generic;
using System.Text;
using ArenaGS.Utilities;
using ProtoBuf;

namespace ArenaGS.Model
{
	public enum MapTheme
	{
		Beach, // sand && stone2_brown
		Sandstone, // sandstone_flooor && sandstone_wall
		FancyInside, // marble_floor && stone2_gray
		Mud, // mud && stone_gray
		Dungeon, // pebble_brown && catacombs
	}
	
	// Technically not immutable, given Set, but promised to be never called outside of MapGen
	[ProtoContract]
	public sealed class Map
	{
		[ProtoMember (1)] // Unless we add destructable terrain, the RNG hash and map name should be sufficient
		public int GenerateHash { get; private set; }
		[ProtoMember (2)]
		public string MapType { get; private set; }

		public int TileHash { get; }
		public int Width { get; }
		public int Height { get; }
		public MapTheme Theme { get;}

		public Map ()
		{
		}

		MapTile[,] Tiles;

		public Map (int width, int height, string mapType, MapTheme theme, int generateHash, int tileHash)
		{
			Width = width;
			Height = height;
			Tiles = new MapTile[width, height];
			MapType = mapType;
			GenerateHash = generateHash;
			TileHash = tileHash;
			Theme = theme;
		}

		public MapTile this [int x, int y] => Tiles[x, y];
		public MapTile this [Point p] => Tiles[p.X, p.Y];

		public bool IsOnMap (Point p) => (p.X >= 0) && (p.Y >= 0) && (p.X < Width) && (p.Y < Height);
		public bool IsOnMap (int x, int y) => (x >= 0) && (y >= 0) && (x < Width) && (y < Height);

		internal void Set (Point p, TerrainType terrain)
		{
			Tiles [p.X, p.Y] = new MapTile (terrain);
		}

		public bool IsWalkable (Point p)
		{
			return IsOnMap (p) && this [p].Walkable;
		}

		public Point CoercePointOntoMap (Point p)
		{
			if (IsOnMap (p))
				return p;
			else
				return new Point (p.X.Clamp (0, Width - 1), p.Y.Clamp (0, Height - 1));
		}

		public override string ToString ()
		{
			StringBuilder output = new StringBuilder ();
			for (int j = 0; j < Height; ++j)
			{
				StringBuilder buffer = new StringBuilder (Width);
				for (int i = 0; i < Width; ++i)
					buffer.Append (Tiles[i, j].Terrain.ToDebugString ());
				output.AppendLine (buffer.ToString ());
			}
			output.AppendLine ();
			return output.ToString ();
		}
	}
}