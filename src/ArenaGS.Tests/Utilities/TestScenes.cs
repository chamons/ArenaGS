using System.Collections.Immutable;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Tests.Utilities
{
	static class TestScenes
	{
		internal static GameState CreateRoomFromMapgen (IGenerator generator)
		{
			var character = generator.CreatePlayer (new Point (1, 1));
			var map = Dependencies.Get<IWorldGenerator> ().GetMapGenerator ("TinyTest").Generate (0).Map;
			var enemies = generator.CreateCharacters (new Point [] { new Point (2, 2) });
			return new GameState (map, character, enemies, ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);
		}
		
		internal static GameState CreateTinyRoomState (IGenerator generator)
		{
			var character = generator.CreatePlayer (new Point (1, 1));
			var map = CreateTinyRoom ();
			var enemies = generator.CreateCharacters (new Point [] { new Point (2, 2) });
			return new GameState (map, character, enemies, ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);
		}

		internal static GameState CreateBoxRoomState (IGenerator generator)
		{
			var character = generator.CreatePlayer (new Point (1, 1));
			var map = CreateBoxRoom (50, 50);
			var enemies = generator.CreateCharacters (new Point [] { new Point (3, 3), new Point (20, 20), new Point (20, 20),
				new Point (40, 20)});
			return new GameState (map, character, enemies, ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);
		}

		internal static GameState CreateWallRoomState (IGenerator generator)
		{
			var character = generator.CreatePlayer (new Point (1, 1));
			var map = CreateBoxRoom (5, 5);
			for (int i = 0 ; i < 5 ; ++i)
				map.Set (new Point(2, i), TerrainType.Wall);
			var enemies = generator.CreateCharacters (new Point [] { new Point (3, 1) });
			return new GameState (map, character, enemies, ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);
		}

		internal static Map CreateTinyRoom ()
		{
			var map = new Map (3, 3, "Tiny", 0);

			for (int i = 0; i < 3; ++i)
				for (int j = 0; j < 3; ++j)
					map.Set (new Point (i, j), TerrainType.Floor);

			map.Set (new Point (1, 2), TerrainType.Wall);
			return map;
		}

		internal static Map CreateBoxRoom (int width, int height)
		{
			Map map = new Map (width, height, "Box", 0);
			for (int i = 1; i < width - 1; ++i)
				for (int j = 1; j < height - 1; ++j)
					map.Set (new Point (i, j), TerrainType.Floor);
			return map;
		}

		internal static Map CreateTinyMaze ()
		{
			Map map = CreateBoxRoom (5, 5);
			map.Set (new Point (2, 1), TerrainType.Wall);
			map.Set (new Point (2, 2), TerrainType.Wall);
			return map;
		}
	}
}
