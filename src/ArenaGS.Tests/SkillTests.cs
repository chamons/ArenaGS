﻿using System;
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
		IPhysics Physics;
		IGenerator Generator;
		ITime Time;
		Skill TestSkill;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Skills = Dependencies.Get<ISkills> ();
			Physics = Dependencies.Get<IPhysics> ();
			Generator = Dependencies.Get<IGenerator> ();
			Time = Dependencies.Get<ITime> ();
			TestSkill = Generator.CreateSkill ("Blast", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 5, 0), SkillResources.None, 1);
		}

		[Test]
		public void UseOfSkill_ReducesInvokersCT ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithSkill (Generator);
			Character enemy = state.Enemies.First (x => x.Position == new Point (3, 3));
			state = state.WithReplaceEnemy (enemy.WithSkills (new Skill [] { TestSkill }.ToImmutableList ()));
			enemy = state.UpdateCharacterReference (enemy);

			Assert.IsTrue (enemy.CT >= 100);
			state = Skills.Invoke (state, enemy, enemy.Skills [0], new Point (2, 3));
			enemy = state.UpdateCharacterReference (enemy);
			Assert.IsTrue (enemy.CT < 100);

			Assert.IsTrue (state.Player.CT >= 100);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 3));
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
		public void CooledBasedAmmoSkill_UnderCooldownButHasAmmo_IsUsable ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithSkillWithResources (Generator, new SkillResources (1, 2, 2, 3, true));
			Assert.IsTrue (state.Player.Skills [0].ReadyForUse);
		}

		[Test]
		public void CooledBasedAmmoSkill_WhenSkillUserIsRemoved_DoesNothing ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			Skill skill = Generator.CreateSkill ("Skill", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 2), SkillResources.WithRechargingAmmo (3, 2), 1);
			state = state.WithReplaceEnemy (state.Enemies [0].WithSkills (skill.Yield ().ToImmutableList ()));

			state = Skills.Invoke (state, state.Enemies [0], state.Enemies [0].Skills [0], new Point (2, 1));
			state = state.WithEnemies (ImmutableList<Character>.Empty);

			for (int i = 0; i < 5; ++i)
			{
				state = Physics.WaitPlayer (state);
				state = Time.ProcessUntilPlayerReady (state);
			}
		}
	}

	[TestFixture]
	class CombatSkillTests
	{
		IGenerator Generator;
		ISkills Skills;
		ITime Time;
		
		CombatStub Combat;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();

			Dependencies.Unregister<ICombat> ();
			Combat = new CombatStub ();
			Dependencies.RegisterInstance<ICombat> (Combat);

			Generator = Dependencies.Get<IGenerator> ();
			Skills = Dependencies.Get<ISkills> ();
			Time = Dependencies.Get<ITime> ();
		}

		[Test]
		public void ActorIsUsingPointSkillValid_DoesDamageToTarget ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithSkill (Generator);

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));

			Character enemyHit = state.Enemies.First (x => x.Position == new Point (3, 3));
			Assert.AreEqual (1, Combat.CharactersDamaged.Count);
			Assert.AreEqual (enemyHit.ID, Combat.CharactersDamaged [0].Item1.ID);
		}

		[Test]
		public void ActorIsUsingPointSkillValid_DoesDamageToSelf ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithSkill (Generator);

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 1));

			Assert.AreEqual (1, Combat.CharactersDamaged.Count);
			Assert.IsTrue (Combat.CharactersDamaged [0].Item1.IsPlayer);
		}

		[Test]
		public void AOESkills_AffectMultipleCharacters ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithAOESkill (Generator);
			var enemies = Generator.CreateStubEnemies (new Point [] { new Point (2, 2), new Point (3, 2)});
			state = state.WithEnemies (enemies.ToImmutableList ());

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 2));
			Assert.AreEqual (3, Combat.CharactersDamaged.Count);
		}

		[Test]
		public void AOESkills_DoNotAffectThroughWalls ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithAOESkill (Generator);
			for (int i = 1; i <= 5; ++i)
				state.Map.Set (new Point (2, i), TerrainType.Wall);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 3));

			// Only player should be damaged, not enemy at 3,3
			Assert.AreEqual (1, Combat.CharactersDamaged.Count);
			Assert.IsTrue (Combat.CharactersDamaged [0].Item1.IsPlayer);
		}

		[Test]
		public void ConeSkills_AffectMultipleCharacters ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithCone (Generator);
			var enemies = Generator.CreateStubEnemies (new Point [] { new Point (2, 1), new Point (2, 2), new Point (2, 3), new Point (3, 3), new Point (1, 5) });
			state = state.WithEnemies (enemies.ToImmutableList ());

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 2));
			Assert.AreEqual (3, Combat.CharactersDamaged.Count);
		}

		[Test]
		public void ConeSkills_DoNotAffectThroughWalls ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithCone (Generator);
			var enemies = Generator.CreateStubEnemies (new Point [] { new Point (4, 1) });
			state = state.WithEnemies (enemies.ToImmutableList ());

			for (int i = 1; i <= 5; ++i)
				state.Map.Set (new Point (3, i), TerrainType.Wall);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 1));

			Assert.AreEqual (0, Combat.CharactersDamaged.Count);
		}

		[Test]
		public void LineSkills_AffectMultipleCharacters ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithLine (Generator);
			var enemies = Generator.CreateStubEnemies (new Point[] { new Point(2, 1), new Point(3, 1), new Point(3, 3), new Point(3, 2)});
			state = state.WithEnemies (enemies.ToImmutableList ());

			state = Skills.Invoke(state, state.Player, state.Player.Skills[0], new Point(2, 1));
			Assert.AreEqual (2, Combat.CharactersDamaged.Count);
		}

		[Test]
		public void Linekills_DoNotAffectThroughWalls ()
		{
			GameState state = TestScenes.CreateBoxRoomStateWithLine (Generator);
			var enemies = Generator.CreateStubEnemies (new Point[] { new Point (3, 1) });
			state = state.WithEnemies (enemies.ToImmutableList ());

			for (int i = 1; i <= 5; ++i)
				state.Map.Set(new Point (2, i), TerrainType.Wall);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 1));

			Assert.AreEqual (0, Combat.CharactersDamaged.Count);
		}

		[Test]
		public void DelayedDamage_DamagesAfterCT ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			Skill delayedDamageSkill = new Skill (1, "Delayed Damage", Effect.DelayedDamage, new TargettingInfo (TargettingStyle.Point, 3), SkillResources.None, 1);
			state = state.WithPlayer (state.Player.WithSkills (delayedDamageSkill.Yield ().ToImmutableList ()));
			state = state.WithEnemies (state.Enemies.Select (x => x.WithCT (-500)).ToImmutableList ());

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));
			state = state.WithPlayer (state.Player.WithCT (-300));
			state = Time.ProcessUntilPlayerReady (state);

			Assert.AreEqual (1, Combat.CharactersDamaged.Count);
			Assert.AreEqual (new Point (3, 3), Combat.CharactersDamaged[0].Item1.Position);
		}
	}
}