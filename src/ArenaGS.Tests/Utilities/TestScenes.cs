using System.Collections.Immutable;
using ArenaGS.Engine;
using ArenaGS.Engine.Generators;
using ArenaGS.Model;
using ArenaGS.Platform;
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
			var map = new Map (3, 3, "Tiny", MapTheme.Mud, 0, 0);

			for (int i = 0; i < 3; ++i)
				for (int j = 0; j < 3; ++j)
					map.Set (new Point (i, j), TerrainType.Floor);

			map.Set (new Point (1, 2), TerrainType.Wall);
			return map;
		}

		internal static Map CreateBoxRoom (int width, int height)
		{
			Map map = new Map (width, height, "Box", MapTheme.Mud, 0, 0);
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

		internal static GameState CreateBoxRoomStateWithSkill (IGenerator generator)
		{
			return AddTestSkill (generator, CreateBoxRoomState (generator));
		}

		internal static GameState AddTestSkill (IGenerator generator, GameState state)
		{
			Skill testSkill = generator.CreateSkill ("Blast", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 5, 0), SkillResources.None);
			return state.WithPlayer (state.Player.WithSkills (new Skill [] { testSkill }.ToImmutableList ()));
		}

		internal static GameState CreateBoxRoomStateWithAOESkill (IGenerator generator)
		{
			return AddTestAOESkill (generator, CreateBoxRoomState (generator));
		}

		internal static GameState AddTestAOESkill (IGenerator generator, GameState state)
		{
			Skill testSkill = generator.CreateSkill ("AOEBlast", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 5, 2), SkillResources.None);
			return state.WithPlayer (state.Player.WithSkills (new Skill [] { testSkill }.ToImmutableList ()));
		}

		internal static GameState CreateBoxRoomStateWithSkillWithResources (IGenerator generator, SkillResources resources)
		{
			GameState state = TestScenes.CreateBoxRoomStateWithSkill (generator);
			return state.WithPlayer (state.Player.WithSkills (state.Player.Skills [0].WithResources (resources).Yield ().ToImmutableList ()));
		}
	}
}
