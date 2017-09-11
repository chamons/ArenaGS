using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

using ArenaGS.Engine.Generators;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;

namespace ArenaGS.Engine
{
	class RoundCoordinator
	{
		internal static GameState Create (IGenerator generator, int round)
		{
			IMapGenerator mapGenerator = Dependencies.Get<IWorldGenerator> ().GetMapGenerator ("OpenArenaMap");
			Random r = new Random ();
			int hash = r.Next ();
			GeneratedMapData mapData = mapGenerator.Generate (hash);

			Point playerPosition = FindOpenSpot (mapData.Map, new Point (8, 8), new Point [] { });
			Character player = generator.CreatePlayer (playerPosition);

			var enemies = CreateEnemies (generator, round, hash, ref mapData, player);
			var startingLog = ImmutableList.Create<string> ();

#if DEBUG
			startingLog = startingLog.Add ($"Map Hash: {hash}");
#endif

			return new GameState (round, mapData.Map, player, enemies, mapData.Scripts, startingLog);
		}

		static ImmutableList<Character> CreateEnemies (IGenerator generator, int round, int hash, ref GeneratedMapData mapData, Character player)
		{
			Random r = new Random (hash);
			int enemiesToCreate = 2 + (round * 2);
			List<Point> enemyPositions = new List<Point> ();
			for (int i = 0; i < enemiesToCreate; ++i)
			{
				Point position = new Point (r.Next (1, mapData.Map.Width), r.Next (1, mapData.Map.Height));
				Point openSpot = FindOpenSpot (mapData.Map, position, enemyPositions.Concat (player.Position.Yield ()));
				if (openSpot != Point.Invalid)
					enemyPositions.Add (openSpot);
			}

			var enemies = enemyPositions.Select (x => generator.CreateCharacter (PickEnemy (r), x)).ToImmutableList ();
			return enemies;
		}

		static string PickEnemy (Random r)
		{
			int val = r.Next (0, 10);
			if (val <= 3)
				return "Wolf";
			else if (val <= 6)
				return "Skeleton";
			else if (val <= 9)
				return "Skeleton Archer";
			else
				return "Golem";
		}


		static Point FindOpenSpot (Map map, Point target, IEnumerable<Point> pointsToAvoid)
		{
			if (map [target].Terrain == TerrainType.Floor && !pointsToAvoid.Contains (target))
				return target;

			for (int i = 0; i < 3; ++i)
			{
				foreach (var point in target.PointsInBurst (i))
				{
					if (map.IsOnMap (point) && map [point].Terrain == TerrainType.Floor && !pointsToAvoid.Contains (point))
						return point;
				}
			}
			return Point.Invalid;
		}
	}
}
