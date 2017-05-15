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
			int width = 15;
			int height = 10;
			Map map = new Map (width, height);
			for (int i = 1; i < width - 1; ++i)
				for (int j = 1; j < height - 1; ++j)
					map.Set (new Point (i, j), TerrainType.Floor);
			return map;
		}
	}
}
