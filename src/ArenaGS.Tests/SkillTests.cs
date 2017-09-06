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
		IPhysics Physics;
		IGenerator Generator;
		ITime Time;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Skills = Dependencies.Get<ISkills> ();
			Physics = Dependencies.Get<IPhysics> ();
			Generator = Dependencies.Get<IGenerator> ();
			Time = Dependencies.Get<ITime> ();
		}

		[Test]
		public void UseOfSkill_ReducesInvokersCT ()
		{
			GameState state = TestScenes.AddTestSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = state.WithEnemies (state.Enemies.Where (x => x.Position == new Point (3, 3)).ToImmutableList ());
			state = TestScenes.AddTestSkill (Generator, state, state.Enemies[0]);

			Assert.IsTrue (state.Enemies [0].CT >= 100);
			state = Skills.Invoke (state, state.Enemies [0], state.Enemies [0].Skills [0], new Point (2, 3));
			Assert.IsTrue (state.Enemies [0].CT < 100);

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
				Skills.Invoke (state, state.Player, TestScenes.CreateSkill (Generator), new Point (2, 2));
			});
		}

		[Test]
		public void SkillsOutOfRange_Throws ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.AddTestSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (10, 10));
			});

			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.AddTestAOESkill (Generator, TestScenes.CreateBoxRoomState (Generator));
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (10, 10));
			});

			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.AddTestConeSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (10, 10));
			});
		}

		[Test]
		public void ActorUsingSkillOffMap_Throws ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.AddTestSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (-10, 10));
			});

			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.AddTestAOESkill (Generator, TestScenes.CreateBoxRoomState (Generator));
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (-10, 10));
			});

			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.AddTestConeSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
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
				GameState state = TestScenes.AddSkillWithResources (Generator, TestScenes.CreateBoxRoomState (Generator), new SkillResources (0, 2, -1, -1, false));
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			});
		}

		[Test]
		public void CooldownSkillNotReadyForUse_ThrowsIfUsed ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.AddSkillWithResources (Generator, TestScenes.CreateBoxRoomState (Generator), new SkillResources (-1, -1, 2, 3, false));
				Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			});
		}

		[Test]
		public void AmmoBasedSkill_ReducesAmmoWhenUsed ()
		{
			GameState state = TestScenes.AddSkillWithResources (Generator, TestScenes.CreateBoxRoomState (Generator), SkillResources.WithAmmo (8));
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			Assert.AreEqual (7, state.Player.Skills [0].Resources.CurrentAmmo);
		}

		[Test]
		public void CooledBasedSkill_SetsCooldownWhenUsed ()
		{
			GameState state = TestScenes.AddSkillWithResources (Generator, TestScenes.CreateBoxRoomState (Generator), SkillResources.WithCooldown (3));
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			Assert.AreEqual (3, state.Player.Skills [0].Resources.Cooldown);
		}

		[Test]
		public void CooledBasedSkillUnderCooldown_ReducesEveryPlayerTurn ()
		{
			const int StartingCooldown = 3;

			GameState state = TestScenes.AddSkillWithResources (Generator, TestScenes.CreateBoxRoomState (Generator), SkillResources.WithCooldown (StartingCooldown));
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

			GameState state = TestScenes.AddSkillWithResources (Generator, TestScenes.CreateBoxRoomState (Generator), SkillResources.WithRechargingAmmo (2, StartingCooldown));
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
			Assert.AreEqual (2, state.Player.Skills [0].Resources.CurrentAmmo);
			Assert.Zero (state.Scripts.Count);
		}

		[Test]
		public void CooledBasedAmmoSkill_UnderCooldownButHasAmmo_IsUsable ()
		{
			GameState state = TestScenes.AddSkillWithResources (Generator, TestScenes.CreateBoxRoomState (Generator), new SkillResources (1, 2, 2, 3, true));
			Assert.IsTrue (state.Player.Skills [0].ReadyForUse);
		}

		[Test]
		public void CooledBasedAmmoSkill_UnderCooldownButHasAmmo_UsedMultipleTimesRefreshesCorrectly ()
		{
			const int StartingCooldown = 3;

			GameState state = TestScenes.AddSkillWithResources (Generator, TestScenes.CreateBoxRoomState (Generator), SkillResources.WithRechargingAmmo (2, StartingCooldown));
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));
			state = Time.ProcessUntilPlayerReady (state);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 1));

			for (int i = 0; i < StartingCooldown; ++i)
			{
				state = Time.ProcessUntilPlayerReady (state);
				state = state.WithPlayer (state.Player.WithCT (0));
			}
			Assert.AreEqual (StartingCooldown, state.Player.Skills [0].Resources.Cooldown);
			Assert.AreEqual (1, state.Player.Skills [0].Resources.CurrentAmmo);

			for (int i = 0; i < StartingCooldown; ++i)
			{
				state = Time.ProcessUntilPlayerReady (state);
				state = state.WithPlayer (state.Player.WithCT (0));
			}
			Assert.AreEqual (0, state.Player.Skills [0].Resources.Cooldown);
			Assert.AreEqual (2, state.Player.Skills [0].Resources.CurrentAmmo);
		}

		[Test]
		public void CooledBasedAmmoSkill_WhenSkillUserIsRemoved_DoesNothing ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			state = TestScenes.AddSkillWithResources (Generator, state, SkillResources.WithRechargingAmmo (3, 2), state.Enemies[0]);

			state = Skills.Invoke (state, state.Enemies [0], state.Enemies [0].Skills [0], new Point (2, 1));
			state = state.WithEnemies (ImmutableList<Character>.Empty);

			for (int i = 0; i < 5; ++i)
			{
				state = Physics.WaitPlayer (state);
				state = Time.ProcessUntilPlayerReady (state);
			}
		}

		[Test]
		public void MovementSkills_ValidTargetInClearLineInRange ()
		{
			GameState state = TestScenes.AddMovementSkill (Generator, TestScenes.CreateBoxRoomState (Generator));

			Assert.IsTrue (Skills.IsValidTarget (state, state.Player, state.Player.Skills [0], new Point (2, 3))); // In range
			Assert.IsFalse (Skills.IsValidTarget (state, state.Player, state.Player.Skills [0], new Point (4, 4))); // In range, blocked line
			Assert.IsFalse (Skills.IsValidTarget (state, state.Player, state.Player.Skills [0], new Point (7, 3))); // Not in range
		}

		[Test]
		public void MovementSkills_MovedInvokerToLocation ()
		{
			GameState state = TestScenes.AddMovementSkill (Generator, TestScenes.CreateBoxRoomState (Generator));

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 3));
			Assert.AreEqual (new Point (2, 3), state.Player.Position);
		}

		[Test]
		public void HealSkillTargettingSelf_IncreasesHealthToMax ()
		{
			GameState state = TestScenes.AddHealSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], state.Player.Position);

			Assert.AreEqual (state.Player.Health.Maximum, state.Player.Health.Current);
		}

		[Test]
		public void MoveAndDamageSkill_DiagonalMove ()
		{
			GameState state = TestScenes.AddMoveAndDamageSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 2));

			Assert.AreEqual (new Point (2, 2), state.Player.Position);
		}

		[Test]
		public void MoveSkill_DiagonalMove ()
		{
			GameState state = TestScenes.AddMovementSkill (Generator, TestScenes.CreateBoxRoomState (Generator), 2);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));

			Assert.AreEqual (new Point (3, 3), state.Player.Position);
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
			GameState state = TestScenes.AddTestSkill (Generator, TestScenes.CreateBoxRoomState (Generator));

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));

			Character enemyHit = state.Enemies.First (x => x.Position == new Point (3, 3));
			Assert.AreEqual (1, Combat.CharactersDamaged.Count);
			Assert.AreEqual (enemyHit.ID, Combat.CharactersDamaged [0].Item1.ID);
		}

		[Test]
		public void ActorIsUsingPointSkillValid_DoesDamageToSelf ()
		{
			GameState state = TestScenes.AddTestSkill (Generator, TestScenes.CreateBoxRoomState (Generator));

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 1));

			Assert.AreEqual (1, Combat.CharactersDamaged.Count);
			Assert.IsTrue (Combat.CharactersDamaged [0].Item1.IsPlayer);
		}

		[Test]
		public void AOESkills_AffectMultipleCharacters ()
		{
			GameState state = TestScenes.AddTestAOESkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			var enemies = Generator.CreateStubEnemies (new Point [] { new Point (2, 2), new Point (3, 2) });
			state = state.WithEnemies (enemies.ToImmutableList ());

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 2));
			Assert.AreEqual (3, Combat.CharactersDamaged.Count);
		}

		[Test]
		public void AOESkills_DoNotAffectThroughWalls ()
		{
			GameState state = TestScenes.AddTestAOESkill (Generator, TestScenes.CreateBoxRoomState (Generator));
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
			GameState state = TestScenes.AddTestConeSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			var enemies = Generator.CreateStubEnemies (new Point [] { new Point (2, 1), new Point (2, 2), new Point (2, 3), new Point (3, 3), new Point (1, 5) });
			state = state.WithEnemies (enemies.ToImmutableList ());

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 2));
			Assert.AreEqual (3, Combat.CharactersDamaged.Count);
		}

		[Test]
		public void ConeSkills_DoNotAffectThroughWalls ()
		{
			GameState state = TestScenes.AddTestConeSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
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
			GameState state = TestScenes.AddTestLineSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			var enemies = Generator.CreateStubEnemies (new Point [] { new Point (2, 1), new Point (3, 1), new Point (3, 3), new Point (3, 2) });
			state = state.WithEnemies (enemies.ToImmutableList ());

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 1));
			Assert.AreEqual (2, Combat.CharactersDamaged.Count);
		}

		[Test]
		public void LineSkills_DoNotAffectThroughWalls ()
		{
			GameState state = TestScenes.AddTestLineSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			var enemies = Generator.CreateStubEnemies (new Point [] { new Point (3, 1) });
			state = state.WithEnemies (enemies.ToImmutableList ());

			for (int i = 1; i <= 5; ++i)
				state.Map.Set (new Point (2, i), TerrainType.Wall);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 1));

			Assert.AreEqual (0, Combat.CharactersDamaged.Count);
		}

		[Test]
		public void DelayedDamage_DamagesAfterCT ()
		{
			GameState state = TestScenes.AddDelayedDamageSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = state.WithEnemies (state.Enemies.Select (x => x.WithCT (-500)).ToImmutableList ());

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));
			state = state.WithPlayer (state.Player.WithCT (-300));
			state = Time.ProcessUntilPlayerReady (state);

			Assert.AreEqual (1, Combat.CharactersDamaged.Count);
			Assert.AreEqual (new Point (3, 3), Combat.CharactersDamaged [0].Item1.Position);
		}

		[Test]
		public void KnockbackSkills_MoveTargetBackIfClear ()
		{
			GameState state = TestScenes.AddKnockbackSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = state.WithEnemies (state.Enemies.Where (x => x.Position == new Point (3, 3)).ToImmutableList ());
			state = state.WithEnemies (state.Enemies.Select (x => x.WithCT (-500)).ToImmutableList ());

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));
			Assert.AreEqual (new Point (4, 4), state.Enemies [0].Position);
		}

		[Test]
		public void KnockbackSkills_DoesNotMoveTargetBackIfBlockedByWall ()
		{
			GameState state = TestScenes.AddKnockbackSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = state.WithEnemies (state.Enemies.Where (x => x.Position == new Point (3, 3)).ToImmutableList ());
			state = state.WithEnemies (state.Enemies.Select (x => x.WithCT (-500)).ToImmutableList ());

			state.Map.Set (new Point (4, 4), TerrainType.Wall);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));
			Assert.AreEqual (new Point (3, 3), state.Enemies [0].Position);
		}

		[Test]
		public void KnockbackSkills_DoesNotMoveTargetBackIfBlockedByCharacter ()
		{
			GameState state = TestScenes.AddKnockbackSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = state.WithEnemies (state.Enemies.Where (x => x.Position == new Point (3, 3)).ToImmutableList ());
			state = state.WithAddedEnemy (Generator.CreateStubEnemy (new Point (4, 4)));
			state = state.WithEnemies (state.Enemies.Select (x => x.WithCT (-500)).ToImmutableList ());

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));
			Assert.AreEqual (new Point (3, 3), state.Enemies [0].Position);
		}

		[Test]
		public void StunSkills_ReduceCTOfTarget ()
		{
			GameState state = TestScenes.AddStunSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = TestStunCore (state);
		}

		[Test]
		public void StunSkillsWithKnockback_ReduceCTOfTarget ()
		{
			GameState state = TestScenes.AddStunSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = state.WithPlayer (state.Player.WithSkills (state.Player.Skills [0].WithEffectInfo (new DamageSkillEffectInfo (1, knockback: true, stun: true)).Yield ().ToImmutableList ()));
			state = TestStunCore (state);
		}

		private GameState TestStunCore (GameState state)
		{
			state = state.WithEnemies (state.Enemies.Where (x => x.Position == new Point (3, 3)).ToImmutableList ());

			Assert.AreEqual (100, state.Enemies [0].CT);
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));
			Assert.Less (state.Enemies [0].CT, 100);
			return state;
		}

		[Test]
		public void ChargeSkills_MoveCasterNextToTargetFirst ()
		{
			GameState state = TestScenes.AddChargeSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));
			Assert.AreEqual (new Point (2, 2), state.Player.Position);
		}

		[Test]
		public void ChargeSkillsNextToTarget_DoesNotMoveCaster ()
		{
			GameState state = TestScenes.AddChargeSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = state.WithPlayer (state.Player.WithPosition (new Point (2, 2)));

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (3, 3));
			Assert.AreEqual (new Point (2, 2), state.Player.Position);
		}

		[Test]
		public void MoveAndDamageClosest_MovesAndDamagesNearest ()
		{
			GameState state = TestScenes.AddMoveAndDamageSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = state.WithEnemies (Generator.CreateStubEnemies (new Point [] { new Point (3, 3), new Point (3, 4) }));

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 1));
			Assert.AreEqual (new Point (2, 1), state.Player.Position);
			Assert.AreEqual (1, Combat.CharactersDamaged.Count);
			Assert.AreEqual (new Point (3, 3), Combat.CharactersDamaged [0].Item1.Position);
		}

		[Test]
		public void MoveAndDamageClosest_IgnoresClosestIfPathIsThroughWall ()
		{
			GameState state = TestScenes.AddMoveAndDamageSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state.Map.Set (new Point (2, 1), TerrainType.Wall);
			state.Map.Set (new Point (2, 2), TerrainType.Wall);

			state = state.WithEnemies (Generator.CreateStubEnemies (new Point [] { new Point (3, 1), new Point (1, 5) }));

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 2));
			Assert.AreEqual (new Point (1, 2), state.Player.Position);
			Assert.AreEqual (1, Combat.CharactersDamaged.Count);
			Assert.AreEqual (new Point (1, 5), Combat.CharactersDamaged [0].Item1.Position);
		}

		[Test]
		public void MoveAndDamageClosest_MovesButNoDamageIfNoEnemiesClose ()
		{
			GameState state = TestScenes.AddMoveAndDamageSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			state = state.WithEnemies (Generator.CreateStubEnemies (new Point [] { new Point (9, 9) }));

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (2, 1));

			Assert.AreEqual (new Point (2, 1), state.Player.Position);
			Assert.AreEqual (0, Combat.CharactersDamaged.Count);
		}

		[Test]
		public void MoveAndDamageClosest_EnemyIgnoresOtherEnemiesWhenLookingForTarget ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = state.WithEnemies (Generator.CreateStubEnemies (new Point [] { new Point (2, 1), new Point (3, 3) }));
			int skilledEnemyID = state.Enemies.First (x => x.Position == new Point (3, 3)).ID;
			state = TestScenes.AddMoveAndDamageSkill (Generator, state, state.Enemies.First (x => x.ID == skilledEnemyID));

			state = Skills.Invoke (state, state.Enemies.First (x => x.ID == skilledEnemyID), state.Enemies.First (x => x.ID == skilledEnemyID).Skills [0], new Point (3, 2));

			Assert.AreEqual (new Point (3, 2), state.Enemies.First (x => x.ID == skilledEnemyID).Position);
			Assert.AreEqual (1, Combat.CharactersDamaged.Count);
			Assert.IsTrue (Combat.CharactersDamaged [0].Item1.IsPlayer);
		}

		[Test]
		public void AreaHealSkills_HealOnlyCharactersOnSide ()
		{
			GameState state = TestScenes.AddHealSkill (Generator, TestScenes.CreateBoxRoomState (Generator));			
			state = state.WithEnemies (Generator.CreateStubEnemies (new Point [] { new Point (1, 2), new Point (2, 1) }));

			state = Skills.Invoke (state, state.Player, state.Player.Skills [0], state.Player.Position);

			Assert.AreEqual (1, Combat.CharactersHealed.Count);
			Assert.IsTrue (Combat.CharactersHealed[0].Item1.IsPlayer);
		}

		[Test]
		public void AreaHealSkills_HealAllValidTargets ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);			
			state = state.WithEnemies (Generator.CreateStubEnemies (new Point [] { new Point (1, 2), new Point (2, 1) }));

			int idOfHealingCharacter = state.Enemies.First (x => x.Position == new Point (1, 2)).ID;
			state = TestScenes.AddHealSkill (Generator, state, state.Enemies.First (x => x.ID == idOfHealingCharacter));

			{
				Character healingCharacter = state.Enemies.First (x => x.ID == idOfHealingCharacter);
				state = Skills.Invoke (state, healingCharacter, healingCharacter.Skills [0], new Point (1, 1));
			}

			Assert.AreEqual (2, Combat.CharactersHealed.Count);
			Assert.IsTrue (Combat.CharactersHealed.All (x => !x.Item1.IsPlayer));
		}

		[Test]
		public void PointsSkillCanEffect_SmokeTest ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = TestScenes.AddTestSkill (Generator, state, state.Player);

			var points = Skills.PointsSkillCanTarget (state, state.Player, state.Player.Skills [0]);
			Assert.AreEqual (25, points.Count);
		}
	}
}