using System;

namespace ArenaGS.Model
{
	public enum TerrainType : byte
	{
		Wall = 0,
		Floor,
		Decoration,
		DecorationSpecial
	}

	public static class TerrainTypeExtensions
	{
		public static string ToDebugString (this TerrainType type)
		{
			switch (type)
			{
				case TerrainType.Decoration:
				case TerrainType.DecorationSpecial:
					return "+";
				case TerrainType.Wall:
					return "#";
				case TerrainType.Floor:
					return ".";
				default:
					throw new NotImplementedException ();
			}
		}
	}

	public struct MapTile
	{
		public TerrainType Terrain { get; private set; }
		public bool Transparent => Terrain == TerrainType.Floor;
		public bool Walkable => Terrain == TerrainType.Floor;

		public MapTile (TerrainType type)
		{
			Terrain = type;
		}
	}
}
