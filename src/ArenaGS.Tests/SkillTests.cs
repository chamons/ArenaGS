using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;
using NUnit.Framework;

namespace ArenaGS.Tests
{
	[TestFixture]
	class SkillTests
	{
		ISkills Skills;
		IGenerator Generator;
		ITime Time;
		Skill TestSkill;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Skills = Dependencies.Get<ISkills> ();
			Generator = Dependencies.Get<IGenerator> ();
			Time = Dependencies.Get<ITime> ();
			TestSkill = Generator.CreateSkill ("Blast", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 5, 0), SkillResources.None);
		}

		[Test]
		public void UseOfSkill_ReducesInvokersCT ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithSkill (Generator);
			Character enemy = state.Enemies.First (x => x.Position == new Point (3, 3));
			state = state.WithReplaceEnemy (enemy.WithSkills (new Skill [] { TestSkill }.ToImmutableList ()));
			enemy = state.UpdateCharacterReference (enemy);

			Assert.IsTrue (enemy.CT >= 100);
			state = Skills.Invoke (state, enemy, enemy.Skills [0], new Point (1, 1));
			enemy = state.UpdateCharacterReference (enemy);
			Assert.IsTrue (enemy.CT < 100);

			Assert.IsTrue (state.Player.CT >= 100);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 1));
			Assert.IsTrue (state.Player.CT < 100);
		}

		[Test]
		public void ActorUsingUnownedSkill_Throws ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateTinyRoomState (Generator);
				Skills.Invoke (state, state.Player, TestSkill, new Point (2, 2));
			});
		}

		[Test]
		public void SkillsOutOfRange_Throws ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateBoxRoomStateWithSkill (Generator);
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (10, 10));
			});

			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateBoxRoomStateWithAOESkill (Generator);
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (10, 10));
			});
		}

		[Test]
		public void ActorUsingSkillOffMap_Throws ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateBoxRoomStateWithSkill (Generator);	
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (-10, 10));
			});

			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateBoxRoomStateWithAOESkill (Generator);
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (-10, 10));
			});

			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateBoxRoomStateWithCone (Generator);
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (-10, 10));
			});
		}

		[Test]
		public void SkillsThroughWall_Throw ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateWallRoomState (Generator);
				state = TestScenes.AddTestSkill (Generator, state);			
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			});

			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateWallRoomState (Generator);
				state = TestScenes.AddTestAOESkill (Generator, state);
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			});

			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateWallRoomState (Generator);
				state = TestScenes.AddTestConeSkill (Generator, state);
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 1));
			});
		}

		[Test]
		public void ConeSkillNotAdjacent_Throws ()
		{
			GameState state = TestScenes.CreateWallRoomState (Generator);
			state = TestScenes.AddTestConeSkill (Generator, state);
			Assert.Throws<InvalidOperationException> (() =>
			{
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			});
			Assert.Throws<InvalidOperationException> (() =>
			{
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 2));
			});
		}

		[Test]
		public void AmmoSkillNotReadyForUse_ThrowsIfUsed ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateBoxRoomStateWithSkillWithResources (Generator, new SkillResources (0, 2, -1, -1, false));
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			});
		}

		[Test]
		public void CooldownSkillNotReadyForUse_ThrowsIfUsed ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateBoxRoomStateWithSkillWithResources (Generator, new SkillResources (-1, -1, 2, 3, false));
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			});
		}

		[Test]
		public void AmmoBasedSkill_ReducesAmmoWhenUsed ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithSkillWithResources (Generator, SkillResources.WithAmmo (8));
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			Assert.AreEqual (7, state.Player.Skills [0].Resources.CurrentAmmo);
		}

		[Test]
		public void CooledBasedSkill_SetsCooldownWhenUsed ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithSkillWithResources (Generator, SkillResources.WithCooldown (3));
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			Assert.AreEqual (3, state.Player.Skills [0].Resources.Cooldown);
		}

		[Test]
		public void CooledBasedSkillUnderCooldown_ReducesEveryPlayerTurn ()
		{
			const int StartingCooldown = 3;

			GameState state = TestScenes.CreateBoxRoomStateWithSkillWithResources (Generator, SkillResources.WithCooldown (StartingCooldown));
			state = state.WithEnemies (ImmutableList<Character>.Empty);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			Assert.AreEqual (StartingCooldown, state.Player.Skills [0].Resources.Cooldown);

			Assert.AreEqual (1, state.Scripts.Count);

			for (int i = 1; i <= StartingCooldown; ++i)
			{
				state = Time.ProcessUntilPlayerReady (state);
				state = state.WithPlayer (state.Player.WithCT (0));
				Assert.AreEqual (StartingCooldown - i, state.Player.Skills [0].Resources.Cooldown);
			}

			Assert.Zero (state.Scripts.Count);
		}

		[Test]
		public void CooledBasedAmmoSkill_IncreasesAmmoOnCooldown ()
		{
			const int StartingCooldown = 3;

			GameState state = TestScenes.CreateBoxRoomStateWithSkillWithResources (Generator, SkillResources.WithRechargingAmmo (StartingCooldown - 1, StartingCooldown));
			state = state.WithEnemies (ImmutableList<Character>.Empty);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			Assert.AreEqual (StartingCooldown, state.Player.Skills [0].Resources.Cooldown);
			Assert.AreEqual (1, state.Player.Skills [0].Resources.CurrentAmmo);

			for (int i = 1; i <= StartingCooldown; ++i)
			{
				state = Time.ProcessUntilPlayerReady (state);
				state = state.WithPlayer (state.Player.WithCT (0));
			}

			Assert.AreEqual (0, state.Player.Skills [0].Resources.Cooldown);
			Assert.AreEqual (StartingCooldown - 1, state.Player.Skills [0].Resources.CurrentAmmo);
			Assert.Zero (state.Scripts.Count);
		}

		[Test]
		public void CooledBasedAmmoSkill_UnderCooldownButHasAmmo_IsUsable()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithSkillWithResources (Generator, new SkillResources (1, 2, 2, 3, true));
			Assert.IsTrue (state.Player.Skills [0].ReadyForUse);

		}
	}

	[TestFixture]
	class SkillTestsWithStubbedPhysics
	{
		public class TestPhysics : IPhysics
		{
			public GameState MovePlayer (GameState state, Direction direction) => throw new NotImplementedException ();
			public GameState MoveEnemy (GameState state, Character enemy, Direction direction) => throw new NotImplementedException ();
			public GameState WaitEnemy (GameState state, Character enemy) => throw new NotImplementedException ();
			public bool CouldCharacterWalk (GameState state, Character actor, Point newPosition) => throw new NotImplementedException ();

			public GameState Wait (GameState state, Character c) => state;
			public GameState WaitPlayer (GameState state) => state;

			public List<Tuple<Character, int>> CharactersDamaged = new List<Tuple<Character, int>> ();
			public GameState Damage (GameState state, Character target, int amount)
			{
				CharactersDamaged.Add (new Tuple<Character, int> (target, amount));
				return state;
			}
		}

		ISkills Skills;
		TestPhysics Physics;
		IGenerator Generator;
		Skill TestSkill;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Dependencies.Unregister<IPhysics> ();

			Physics = new TestPhysics ();
			Dependencies.RegisterInstance<IPhysics> (Physics);

			Skills = Dependencies.Get<ISkills> ();
			Generator = Dependencies.Get<IGenerator>();
			TestSkill = Generator.CreateSkill ("Blast", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 5, 0), SkillResources.None);
		}

		[Test]
		public void ActorIsUsingPointSkillValid_DoesDamageToTarget ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithSkill (Generator);

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));

			Character enemyHit = state.Enemies.First (x => x.Position == new Point (3, 3));
			Assert.AreEqual (1, Physics.CharactersDamaged.Count);
			Assert.AreEqual (enemyHit.ID, Physics.CharactersDamaged [0].Item1.ID);
		}

		[Test]
		public void ActorIsUsingPointSkillValid_DoesDamageToSelf ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithSkill (Generator);

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 1));

			Assert.AreEqual (1, Physics.CharactersDamaged.Count);
			Assert.IsTrue (Physics.CharactersDamaged[0].Item1.IsPlayer);
		}

		[Test]
		public void AOESkills_AffectMultipleCharacters ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithAOESkill (Generator);

			state = Skills.Invoke (state, state.Player, state.Player.Skills[0], new Point (2, 2));
			Assert.AreEqual (2, Physics.CharactersDamaged.Count);
		}

		[Test]
		public void AOESkills_DoNotAffectThroughWalls ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithAOESkill (Generator);
			for (int i = 1 ; i <= 5; ++i)
				state.Map.Set (new Point (2, i), TerrainType.Wall);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 3));

			// Only player should be damaged, not enemy at 3,3
			Assert.AreEqual (1, Physics.CharactersDamaged.Count);
			Assert.IsTrue (Physics.CharactersDamaged[0].Item1.IsPlayer);
		}

		[Test]
		public void ConeSkills_AffectMultipleCharacters ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithCone (Generator);
			var enemies = Generator.CreateCharacters (new Point [] { new Point (2, 1), new Point (2, 2), new Point (2, 3), new Point (3, 3), new Point (1, 5) });
			state = state.WithEnemies (enemies.ToImmutableList ());

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 2));
			Assert.AreEqual (2, Physics.CharactersDamaged.Count);
		}

		[Test]
		public void ConeSkills_DoNotAffectThroughWalls ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithCone (Generator);
			var enemies = Generator.CreateCharacters (new Point [] { new Point (4, 1) });
			state = state.WithEnemies (enemies.ToImmutableList ());

			for (int i = 1; i <= 5; ++i)
				state.Map.Set (new Point (3, i), TerrainType.Wall);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 1));

			Assert.AreEqual (0, Physics.CharactersDamaged.Count);
		}
	}
}