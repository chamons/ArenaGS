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
			var character = generator.CreateTestPlayer (new Point (1, 1));
			var map = Dependencies.Get<IWorldGenerator> ().GetMapGenerator ("TinyTest").Generate (0).Map;
			var enemies = TestEnemyHelper.CreateTestEnemies (generator, new Point [] { new Point (2, 2) });
			return new GameState (map, character, enemies, ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);
		}
		
		internal static GameState CreateTinyRoomState (IGenerator generator)
		{
			var character = generator.CreateTestPlayer (new Point (1, 1));
			var map = CreateTinyRoom ();
			var enemies = TestEnemyHelper.CreateTestEnemies (generator, new Point [] { new Point (2, 2) });
			return new GameState (map, character, enemies, ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);
		}

		internal static GameState CreateBoxRoomState (IGenerator generator)
		{
			var character = generator.CreateTestPlayer (new Point (1, 1));
			var map = CreateBoxRoom (50, 50);
			var enemies = TestEnemyHelper.CreateTestEnemies (generator, (new Point [] { new Point (3, 3), new Point (20, 20), new Point (20, 20),
				new Point (40, 20)}));
			return new GameState (map, character, enemies, ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);
		}

		internal static GameState CreateWallRoomState (IGenerator generator)
		{
			var character = generator.CreateTestPlayer (new Point (1, 1));
			var map = CreateBoxRoom (5, 5);
			for (int i = 0 ; i < 5 ; ++i)
				map.Set (new Point(2, i), TerrainType.Wall);
			var enemies = TestEnemyHelper.CreateTestEnemies (generator, (new Point [] { new Point (3, 1) }));
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

		internal static Skill CreateSkill (IGenerator generator)
		{
			return generator.CreateSkill ("Blast", Effect.Damage, new DamageSkillEffectInfo (1), TargettingInfo.Point (5), SkillResources.None);
		}

		internal static GameState AddTestSkill (IGenerator generator, GameState state, Character character = null)
		{
			if (character == null)
				character = state.Player;
			Skill testSkill = CreateSkill (generator);
			return state.WithReplaceCharacter (character.WithAdditionalSkill (testSkill));
		}

		internal static GameState AddTestAOESkill (IGenerator generator, GameState state)
		{
			Skill testSkill = generator.CreateSkill ("AOEBlast", Effect.Damage, new DamageSkillEffectInfo (1), TargettingInfo.Point (5, 2), SkillResources.None);
			return state.WithPlayer (state.Player.WithAdditionalSkill (testSkill));
		}

		internal static GameState AddTestConeSkill (IGenerator generator, GameState state)
		{
			Skill testSkill = generator.CreateSkill ("Cone", Effect.Damage, new DamageSkillEffectInfo (1), TargettingInfo.Cone (3), SkillResources.None);
			return state.WithPlayer (state.Player.WithAdditionalSkill (testSkill));
		}

		internal static GameState AddTestLineSkill (IGenerator generator, GameState state)
		{
			Skill testSkill = generator.CreateSkill ("Line", Effect.Damage, new DamageSkillEffectInfo (1), TargettingInfo.Line (3), SkillResources.None);
			return state.WithPlayer (state.Player.WithAdditionalSkill (testSkill));
		}

		internal static GameState AddDelayedDamageSkill (IGenerator generator, GameState state)
		{
			Skill testSkill = new Skill (1, "Delayed Damage", Effect.DelayedDamage, new DelayedDamageSkillEffectInfo (1), TargettingInfo.Point (3), SkillResources.None);
			return state.WithPlayer (state.Player.WithAdditionalSkill (testSkill));
		}

		internal static GameState AddSkillWithResources (IGenerator generator, GameState state, SkillResources resources, Character character = null)
		{
			if (character == null)
				character = state.Player;
			state = AddTestSkill (generator, state, character);
			character = state.UpdateCharacterReference (character); //(╯°□°）╯︵ ┻━┻
			return state.WithReplaceCharacter (character.WithReplaceSkill (character.Skills [0].WithResources (resources)));
		}

		internal static GameState AddMovementSkill (IGenerator generator, GameState state, int range = 5)
		{
			Skill testSkill = generator.CreateSkill ("Dash", Effect.Movement, SkillEffectInfo.None, TargettingInfo.Point (range), SkillResources.WithCooldown (3));
			return state.WithPlayer (state.Player.WithAdditionalSkill (testSkill));
		}

		internal static GameState AddKnockbackSkill (IGenerator generator, GameState state)
		{
			Skill testSkill = generator.CreateSkill ("Knockback", Effect.Damage, new DamageSkillEffectInfo (2, knockback: true), TargettingInfo.Point (5), SkillResources.WithCooldown (3));
			return state.WithPlayer (state.Player.WithAdditionalSkill (testSkill));
		}

		internal static GameState AddStunSkill (IGenerator generator, GameState state)
		{
			Skill testSkill = generator.CreateSkill ("Stun", Effect.Damage, new DamageSkillEffectInfo (2, stun: true), TargettingInfo.Point (5), SkillResources.WithCooldown (3));
			return state.WithPlayer (state.Player.WithAdditionalSkill (testSkill));
		}

		internal static GameState AddChargeSkill (IGenerator generator, GameState state)
		{
			Skill testSkill = generator.CreateSkill ("Charge", Effect.Damage, new DamageSkillEffectInfo (3, charge: true), TargettingInfo.Point (5), SkillResources.WithCooldown (3));
			return state.WithPlayer (state.Player.WithAdditionalSkill (testSkill));
		}

		internal static GameState AddMoveAndDamageSkill (IGenerator generator, GameState state, Character character = null)
		{
			if (character == null)
				character = state.Player;
			Skill testSkill = generator.CreateSkill ("Move & Shoot", Effect.MoveAndDamageClosest, new MoveAndDamageSkillEffectInfo (3, 3), TargettingInfo.Point (1), SkillResources.WithCooldown (3));
			return state.WithReplaceCharacter (character.WithAdditionalSkill (testSkill));
		}

		internal static GameState AddHealSkill (IGenerator generator, GameState state, Character character = null)
		{
			if (character == null)
				character = state.Player;
			Skill testSkill = generator.CreateSkill ("Heal", Effect.Heal, new HealEffectInfo (3), TargettingInfo.Point (2, 2), SkillResources.WithCooldown (3));
			return state.WithReplaceCharacter (character.WithAdditionalSkill (testSkill));
		}
	}
}
