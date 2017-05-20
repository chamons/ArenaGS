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
			var map = Dependencies.Get<IWorldGenerator> ().GetMapGenerator ("TinyTest").Generate (0);
			var enemies = ImmutableList.Create (new Character[] { Character.Create (new Point (2, 2)) });
			return new GameState (map, character, enemies, ImmutableList<string>.Empty);
		}

		internal static Map CreateBoxRoom (int width, int height)
		{
			Map map = new Map (width, height, "Box", 0);
			for (int i = 1; i < width - 1; ++i)
				for (int j = 1; j < height - 1; ++j)
					map.Set (new Point (i, j), TerrainType.Floor);
			return map;
		}
	}
}
