using ProtoBuf;

namespace ArenaGS.Model
{
	public enum TerrainType : byte
	{
		Wall = 0,
		Floor
	}

	[ProtoContract]
	public struct MapTile
	{
		[ProtoMember (1)]
		public TerrainType Terrain { get; private set; }

		public MapTile (TerrainType type)
		{
			Terrain = type;
		}
	}
}
