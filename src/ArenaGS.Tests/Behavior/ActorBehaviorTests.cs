using System.Collections.Immutable;
using System.Linq;

using ArenaGS.Engine;
using ArenaGS.Engine.Behavior;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;

using NUnit.Framework;
using System.Collections.Generic;

namespace ArenaGS.Tests
{
	[TestFixture]
	class ActorBehaviorTests
	{
		IPhysics Physics;
		ITime Time;
		IGenerator Generator;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Physics = Dependencies.Get<IPhysics> ();
			Time = Dependencies.Get<ITime> ();
			Generator = Dependencies.Get<IGenerator> ();
		}

		[Test]
		public void DefaultActorBehavior_MovesTowardsPlayer_UnlessNextTo ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			DefaultActorBehavior behavior = new DefaultActorBehavior ();
			// Player is at 1,1. Enemy is at 3,3
			Character closest = state.Enemies.First (x => x.Position == new Point (3, 3));

			// First movement should move to 2,2
			state = behavior.Act (state, closest);
			closest = state.UpdateCharacterReference (closest);
			Assert.AreEqual (closest.Position, new Point (2, 2));

			// After getting CT, second move should not move closer
			state = state.WithEnemies (state.Enemies.Select (x => x.WithAdditionalCT (TimeConstants.CTNededForAction)).ToImmutableList ());
			closest = state.UpdateCharacterReference (closest);

			state = behavior.Act (state, closest);
			closest = state.UpdateCharacterReference (closest);
			Assert.AreEqual (closest.Position, new Point (2, 2));
		}

		[Test]
		public void DefaultActorBehavior_BlockedEnemy_Waits ()
		{
			Map map = TestScenes.CreateBoxRoom (5, 5);
			Character player = Generator.CreateTestPlayer (new Point (1, 1));
			// W
			//  P E E
			//  E E E
			//  E E E
			var enemies = TestEnemyHelper.CreateTestEnemies (Generator, new Point [] { new Point (2, 1), new Point (3, 1), new Point (1, 2),
				new Point (2, 2), new Point (2, 3), new Point (1, 3), new Point (2, 3), new Point (3,3)});
			GameState state = new GameState (map, player, enemies, ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);

			Character blockedCharacter = enemies.First (x => x.Position == new Point (3, 3));
			DefaultActorBehavior behavior = new DefaultActorBehavior ();
			state = behavior.Act (state, blockedCharacter);
			blockedCharacter = state.UpdateCharacterReference (blockedCharacter);

			Assert.AreEqual (blockedCharacter.Position, new Point (3, 3));
			Assert.AreEqual (blockedCharacter.CT, 0);
		}

		[Test]
		public void DefaultActorBehavior_MultipleTurnMove_TowardsPlayer ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			var shortestPath = state.ShortestPath;

			Dictionary<int, int> startingEnemyDistance = new Dictionary<int, int> ();
			foreach (var enemy in state.Enemies)
				startingEnemyDistance.Add (enemy.ID, shortestPath [enemy.Position.X, enemy.Position.Y]);

			for (int i = 0; i < 10; ++i)
			{
				state = Physics.WaitPlayer (state);
				state = Time.ProcessUntilPlayerReady (state);
			}

			foreach (var enemy in state.Enemies)
				Assert.Less (shortestPath [enemy.Position.X, enemy.Position.Y], startingEnemyDistance [enemy.ID]);
		}

		[Test]
		public void UsesMovementSkill_ToCloseGameWithPlayer ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			Skill movementSkill = Generator.CreateSkill ("TestDash", Effect.Movement, SkillEffectInfo.None, TargettingInfo.Point (3), SkillResources.WithCooldown (2));
			Character enemy = Generator.CreateCharacter ("TestEnemy", new Point (4, 4)).WithSkills (movementSkill.YieldList ());
			state = state.WithCharacters (enemy.Yield ());
			enemy = state.UpdateCharacterReference (enemy);

			var shortestPath = state.ShortestPath;
			Assert.AreEqual (3, shortestPath [enemy.Position.X, enemy.Position.Y]);

			DefaultActorBehavior behavior = new DefaultActorBehavior ();
			state = behavior.Act (state, enemy);
			enemy = state.UpdateCharacterReference (enemy);

			Assert.AreEqual (1, shortestPath [enemy.Position.X, enemy.Position.Y]);
			Assert.False (enemy.Skills [0].ReadyForUse);
		}
	}

	[TestFixture]
	class CombatActorBehaviorTests
	{
		IGenerator Generator;
		ISkills Skills;
		ITime Time;
		IPhysics Physics;

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
			Physics = Dependencies.Get<IPhysics> ();
		}

		Skill GetTestBite () => Generator.CreateSkill ("TestBite", Effect.Damage, new DamageSkillEffectInfo (2), TargettingInfo.Point (1), SkillResources.WithCooldown (2));
		Skill GetStrongTestBite () => Generator.CreateSkill ("TestStrongBite", Effect.Damage, new DamageSkillEffectInfo (3), TargettingInfo.Point (1), SkillResources.WithCooldown (2));
		Skill GetTestShot () => Generator.CreateSkill ("TestShot", Effect.Damage, new DamageSkillEffectInfo (2), TargettingInfo.Point (3), SkillResources.WithCooldown (2));
		Skill GetStrongTestShot () => Generator.CreateSkill ("TestStrongShot", Effect.Damage, new DamageSkillEffectInfo (3), TargettingInfo.Point (3), SkillResources.WithCooldown (2));
		Skill GetStunBite () => Generator.CreateSkill ("StunBite", Effect.Damage, new DamageSkillEffectInfo (1, stun: true), TargettingInfo.Point (1), SkillResources.WithCooldown (2));
		Skill GetLineAttack () => Generator.CreateSkill ("Line Attack", Effect.Damage, new DamageSkillEffectInfo (2), TargettingInfo.Line (3), SkillResources.WithCooldown (2));
		Skill GetAreaAttack () => Generator.CreateSkill ("Area Attack", Effect.Damage, new DamageSkillEffectInfo (2), TargettingInfo.Point (3, 1), SkillResources.WithCooldown (2));
		Skill GetConeAttack () => Generator.CreateSkill ("Clone Attack", Effect.Damage, new DamageSkillEffectInfo (2), TargettingInfo.Cone (3), SkillResources.WithCooldown (2));
		Skill GetMoveAndShoot () => Generator.CreateSkill ("Move & Shoot", Effect.MoveAndDamageClosest, new MoveAndDamageSkillEffectInfo (1, 3), TargettingInfo.Point (1), SkillResources.WithCooldown (2));
		Skill GetDelayedBlast () => Generator.CreateSkill ("Delayed Blast", Effect.DelayedDamage, new DelayedDamageSkillEffectInfo (4), TargettingInfo.Line (4), SkillResources.WithCooldown (2));


		GameState AddEnemyWithSkills (GameState state, IEnumerable<Skill> skills, Point position)
		{
			Character enemy = Generator.CreateCharacter ("TestEnemy", position).WithSkills (skills.ToImmutableList ());
			return state.WithCharacters (enemy.Yield ());
		}

		GameState AddEnemyWithSkills (GameState state, IEnumerable<Skill> skills)
		{
			return AddEnemyWithSkills (state, skills, new Point (2, 1));
		}

		GameState ActFirstEnemy (GameState state)
		{
			DefaultActorBehavior behavior = new DefaultActorBehavior ();
			return behavior.Act (state, state.Enemies [0]);
		}

		static void AssertSkillUsed (GameState state, string skillName)
		{
			Assert.IsFalse (state.Enemies [0].Skills.First (x => x.Name == skillName).ReadyForUse);
		}

		[Test]
		public void UsesAttackSkill_WhenInRangeOfPlayer ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetTestBite () });

			state = ActFirstEnemy (state);

			Assert.IsFalse (state.Enemies [0].Skills [0].ReadyForUse);
			Assert.AreEqual (Combat.CharactersDamaged.Count, 1);
			Assert.IsTrue (Combat.CharactersDamaged [0].Item1.IsPlayer);
		}

		[Test]
		public void UsesLineAttackSkill_WhenInRangeOfPlayer ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetLineAttack () });

			state = ActFirstEnemy (state);

			Assert.IsFalse (state.Enemies [0].Skills [0].ReadyForUse);
			Assert.AreEqual (Combat.CharactersDamaged.Count, 1);
			Assert.IsTrue (Combat.CharactersDamaged [0].Item1.IsPlayer);
		}

		[Test]
		public void DoesNotUseLineAttackSkill_WhenNotInRange ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetLineAttack () }, new Point (10, 10));

			state = ActFirstEnemy (state);

			Assert.True (state.Enemies [0].Skills [0].ReadyForUse);
		}

		[Test]
		public void UsesConeAttackSkill_WhenInRangeOfPlayer ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetConeAttack () });

			state = ActFirstEnemy (state);

			Assert.IsFalse (state.Enemies [0].Skills [0].ReadyForUse);
			Assert.AreEqual (Combat.CharactersDamaged.Count, 1);
			Assert.IsTrue (Combat.CharactersDamaged [0].Item1.IsPlayer);
		}

		[Test]
		public void DoesNotUseConeAttackSkill_WhenNotInRange ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetConeAttack () }, new Point (10, 10));

			state = ActFirstEnemy (state);

			Assert.True (state.Enemies [0].Skills [0].ReadyForUse);
		}

		[Test]
		public void UsesStongestAttackSkill_WhenMultipleAvailable ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetTestBite (), GetStrongTestBite () });

			state = ActFirstEnemy (state);

			AssertSkillUsed (state, "TestStrongBite");
		}

		[Test]
		public void UsesMovementAttackSkill_InPreferenceToRegular_WhenAvailableMovesAway ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetStrongTestShot (), GetMoveAndShoot () });
			int startingDistance = state.ShortestPath [state.Enemies [0].Position.X, state.Enemies [0].Position.Y];

			state = ActFirstEnemy (state);

			AssertSkillUsed (state, "Move & Shoot");
			int endingDistance = state.ShortestPath [state.Enemies [0].Position.X, state.Enemies [0].Position.Y];
			// Strongest skill is shot, so should move away
			Assert.Greater (endingDistance, startingDistance);
		}

		[Test]
		public void UsesMovementAttackSkill_InPreferenceToRegular_WhenAvailableMovesTowards ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetStrongTestBite (), GetMoveAndShoot () }, new Point (3, 3));
			int startingDistance = state.ShortestPath [state.Enemies [0].Position.X, state.Enemies [0].Position.Y];

			state = ActFirstEnemy (state);

			AssertSkillUsed (state, "Move & Shoot");
			int endingDistance = state.ShortestPath [state.Enemies [0].Position.X, state.Enemies [0].Position.Y];
			// Strongest skill is bite, so should move towards
			Assert.Less (endingDistance, startingDistance);
		}

		[Test]
		public void UsesStunAttackSkill_InPreference_WhenAvailable ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetTestBite (), GetStunBite () });

			state = ActFirstEnemy (state);

			AssertSkillUsed (state, "StunBite");
		}

		[Test]
		public void PreferNonDelayedAttackSkills_WhenBothAvailable ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetTestBite (), GetDelayedBlast () });

			state = ActFirstEnemy (state);

			AssertSkillUsed (state, "TestBite");
		}

		[Test]
		public void UsesDelayedAttackSkill_WhenInRangeOfPlayer_WhenOnlyOption ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetDelayedBlast () });

			state = ActFirstEnemy (state);

			AssertSkillUsed (state, "DelayedBlast");

			for (int i = 0; i < 3; ++i)
			{
				state = Physics.Wait (state, state.Player);
				state = Physics.Wait (state, state.Enemies[0]);
				state = Time.ProcessUntilPlayerReady (state);
			}
			Assert.AreEqual (1, Combat.CharactersDamaged.Count);
			Assert.True (Combat.CharactersDamaged[0].Item1.IsPlayer);
		}

		[Test]
		public void DoesNotUsesDelayedAttackSkill_WhenNotInRangeOfPlayer ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = AddEnemyWithSkills (state, new Skill [] { GetDelayedBlast () }, new Point (10, 10));

			state = ActFirstEnemy (state);

			Assert.True (state.Enemies [0].Skills [0].ReadyForUse);
		}

		[Test]
		public void UsesAreaAttackSkill_WhenDirectlyInRangeOfPlayer ()
		{
			//Assert.Fail ();
		}

		[Test]
		public void UsesAreaAttackSkill_WhenSplashInRangeOfPlayer ()
		{
			//Assert.Fail ();
		}

		[Test]
		public void UsesAreaAttackSkill_ToNotHitOthersWhenPossible ()
		{
			//Assert.Fail ();
		}

		[Test]
		public void UsesKnockbackSkill_WhenRanged_AndAvailable ()
		{
			//Assert.Fail ();
		}

		[Test]
		public void UsesSelfHeal_OnlyWhenDamaged_AndAvailable ()
		{
			//Assert.Fail ();
		}

		[Test]
		public void RangedEnemiesFallBack_OnlyIfMultipleEnemiesNearPlayer_AreAbleToMove ()
		{
			//Assert.Fail ();
		}
	}
}
