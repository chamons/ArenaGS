using ArenaGS.Utilities;

namespace ArenaGS.Model
{
	internal interface IMapGenerator
	{
		Map Generate ();
	}
	
	internal class SimpleMapGenerator : IMapGenerator
	{
		public Map Generate ()
		{
			Map map = new Map (40, 40);
			for (int i = 0; i < 40; ++i)
				for (int j = 0; j < 40; ++j)
					map.Set (new Point (i, j), i % 2 == 0 ? TerrainType.Floor : TerrainType.Wall);
			return map;
		}
	}
}
