using System;

namespace ArenaGS.Model
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
				case "Simple":
					return new SimpleMapGenerator ();
				case "OpenArenaMap":
					return new OpenArenaMapGenerator ();
				default:
					throw new NotImplementedException ();
			}
		}
	}
}
