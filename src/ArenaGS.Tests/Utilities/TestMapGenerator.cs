﻿using System;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Tests.Utilities
{
	class TestWorldGenerator : IWorldGenerator
	{
		public IMapGenerator GetMapGenerator (string type)
		{
			switch (type)
			{
				case "TinyTest":
					return new TinyRoomTestMapGenerator ();
				default:
					throw new NotImplementedException ();
			}
		}
	}

	class TinyRoomTestMapGenerator : IMapGenerator
	{
		public Map Generate (int hash)
		{
			var map = new Map (3, 3, "TinyTest", hash);

			for (int i = 0; i < 3; ++i)
				for (int j = 0; j < 3; ++j)
					map.Set (new Point (i, j), TerrainType.Floor);

			map.Set (new Point (1, 2), TerrainType.Wall);

			return map;
		}

		public Map Regenerate (int hash)
		{
			return Generate (hash);
		}
	}
}
