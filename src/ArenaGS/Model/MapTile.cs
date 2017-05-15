namespace ArenaGS.Model
{
	public enum TerrainType : byte
	{
		Wall = 0,
		Floor
	}

	public struct MapTile
	{
		public TerrainType Terrain { get; private set; }

		public MapTile (TerrainType type)
		{
			Terrain = type;
		}
	}
}
