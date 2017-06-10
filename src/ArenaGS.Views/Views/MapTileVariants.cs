﻿using System;
using System.Collections.Generic;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Views.Views
{
	class MapTileVariants
	{
		Dictionary <int, byte []> Hashes = new Dictionary<int, byte[]> ();

		// This is not super fast - https://github.com/chamons/ArenaGS/issues/49
		internal int Get (Map map, Point position, int variants, int startAt = 0, int rareAbove = -1)
		{
			byte [] hash;
			if (!Hashes.TryGetValue (map.TileHash, out hash))
			{
				Random rng = new Random (map.TileHash);
				hash = new byte [map.Width * map.Height];
				rng.NextBytes (hash);
			}

			if (rareAbove == -1)
			{
				return startAt + hash [position.X + map.Width * position.Y] % variants;
			}
			else
			{
				// Extend the range of the "common" numbers to make them more common
				// Example: 6 variants, start at 1, rare above 3
				//  Was: 0 1 2 3 4 5 + 1
				//  Now: 0 1 2 0 1 2 0 1 2 3  4  5 + 1
				// Math: 0 1 2 3 4 5 6 7 8 9 10 11 => if (x < 6) { x % 3 + 1 } else { x - 6 + 1 }
				// Example: 4 variants, rare above 1
				//  Was: 0 1 2 3
				//  Now: 0 1 0 1 0 1 2 3
				// Math: 0 1 2 3 4 5 6 7 => if (x < 4) { x % 2 } else { x - 4 }
				int commonRangeLength = variants - rareAbove + startAt - 1;
				int extendedVariantRange = (commonRangeLength * 3) + (variants - commonRangeLength);

				int extendedHash = hash [position.X + map.Width * position.Y] % extendedVariantRange;
				if (extendedHash < commonRangeLength * 2)
					return (extendedHash % commonRangeLength) + startAt;
				else
					return (extendedHash - (commonRangeLength * 2)) + startAt;

			}
		}
	}
}
