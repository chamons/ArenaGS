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
			Character enemy = Generator.CreateCharacter ("TestEnemy", new Point (4, 4)).WithSkills (movementSkill.Yield ().ToImmutableList ());
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
		public void UsesAttackSkill_WhenInRangeOfPlayer ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			Skill damageSkill = Generator.CreateSkill ("TestBite", Effect.Damage, new DamageSkillEffectInfo (1), TargettingInfo.Point (1), SkillResources.WithCooldown (2));
			Character enemy = Generator.CreateCharacter ("TestEnemy", new Point (2, 1)).WithSkills (damageSkill.Yield ().ToImmutableList ());
			state = state.WithCharacters (enemy.Yield ());
			enemy = state.UpdateCharacterReference (enemy);

			DefaultActorBehavior behavior = new DefaultActorBehavior ();
			state = behavior.Act (state, enemy);
			enemy = state.UpdateCharacterReference (enemy);

			Assert.IsFalse (enemy.Skills [0].ReadyForUse);
			Assert.AreEqual (Combat.CharactersDamaged.Count, 1);
			Assert.IsTrue (Combat.CharactersDamaged [0].Item1.IsPlayer);
		}

		[Test]
		public void UsesMovementAttackSkill_WhenInRangeOfPlayer_AndKeepsDistance ()
		{
			//Assert.Fail ();
		}

		[Test]
		public void UsesMovementAttackSkill_InPreferenceToRegular_WhenAvailable ()
		{
			//Assert.Fail ();
		}

		[Test]
		public void UsesStunAttackSkill_InPreference_WhenAvailable ()
		{
			//Assert.Fail ();
		}

		[Test]
		public void UsesDelayedAttackSkill_WhenInRangeOfPlayer_WhenOnlyOption ()
		{
			//Assert.Fail ();
		}

		[Test]
		public void UsesSelfHeal_OnlyWhenDamaged_AndAvailable ()
		{
		}
	}
}
