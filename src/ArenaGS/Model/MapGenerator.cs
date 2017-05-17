using System;
using ArenaGS.Utilities;

namespace ArenaGS.Model
{
	internal interface IMapGenerator
	{
		Map Generate (int hash);

		Map Regenerate (int hash);
	}
	
	internal class SimpleMapGenerator : IMapGenerator
	{
		public Map Generate (int hash)
		{
			int width = 15;
			int height = 10;
			Map map = new Map (width, height, "Simple", hash);
			for (int i = 1; i < width - 1; ++i)
				for (int j = 1; j < height - 1; ++j)
					map.Set (new Point (i, j), TerrainType.Floor);
			return map;
		}

		public Map Regenerate (int hash)
		{
			return Generate (hash);
		}
	}
}
