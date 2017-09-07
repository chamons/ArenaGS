﻿using System;

namespace ArenaGS.Engine.Generators
{
	interface IWorldGenerator
	{
		IMapGenerator GetMapGenerator (string type);
	}

	class WorldGenerator : IWorldGenerator
	{
		public IMapGenerator GetMapGenerator (string type)
		{
			switch (type)
			{
				case "OpenArenaMap":
					return new OpenArenaMapGenerator ();
				default:
					throw new NotImplementedException ();
			}
		}
	}
}
