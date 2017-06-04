using System.Collections.Immutable;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Tests.Utilities
{
	static class TestScenes
	{
		internal static GameState CreateTinyRoomState ()
		{
			var character = Character.CreatePlayer (new Point (1, 1)).WithCT (100);
			var mapData = Dependencies.Get<IWorldGenerator> ().GetMapGenerator ("TinyTest").Generate (0);
			var enemies = ImmutableList.Create (new Character[] { Character.Create (new Point (2, 2)) });
			return new GameState (mapData.Map, character, enemies, ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);
		}

		internal static GameState CreateBoxRoomState ()
		{
			var character = Character.CreatePlayer (new Point (1, 1)).WithCT (100);
			var map = CreateBoxRoom (50, 50);
			var enemies = ImmutableList.Create (new Character[] { Character.Create (new Point (3, 3)), Character.Create (new Point (20, 20)), Character.Create (new Point (5, 40)), Character.Create (new Point (40, 20))});
			return new GameState (map, character, enemies, ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);
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
