using System;
using System.Collections.Generic;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Views.Views
{
	class MapTileVariants
	{
		Dictionary <int, byte []> Hashes = new Dictionary<int, byte[]> ();

		// This is not super fast - https://github.com/chamons/ArenaGS/issues/49
		internal int Get (Map map, Point position)
		{
			byte [] hash;
			if (!Hashes.TryGetValue (map.TileHash, out hash))
			{
				Random rng = new Random (map.TileHash);
				hash = new byte [map.Width * map.Height];
				rng.NextBytes (hash);
			}
			return hash [position.X + map.Width * position.Y] % 4;
		}
	}
}
