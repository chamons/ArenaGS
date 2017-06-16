using System.Collections.Immutable;

using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Tests.Utilities;
using NUnit.Framework;
using ArenaGS.Utilities;
using System;

namespace ArenaGS.Tests
{
	[TestFixture]
	public class CombatTests : IAnimationRequest
	{
		ICombat Combat;
		IGenerator Generator;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Dependencies.Unregister<IAnimationRequest> ();
			Dependencies.RegisterInstance<IAnimationRequest> (this);

			Combat = Dependencies.Get<ICombat> ();
			Generator = Dependencies.Get<IGenerator> ();
		}

		public void Request (GameState state, AnimationInfo info) {}

		GameState PlayerDeathState ;
		public void RequestPlayerDead (GameState state)
		{
			PlayerDeathState = state;
		}

		[Test]
		public void DamagedCharacter_HasFewerHP ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = state.WithEnemies (Generator.CreateCharacter (new Point (2, 2), new Health (10, 10), new Defense (0)).Yield ().ToImmutableList ());

			Character enemy = state.Enemies [0];
			state = Combat.Damage (state, enemy, 1);
			enemy = state.UpdateCharacterReference (enemy);

			Assert.Less (enemy.Health.Current, 10);
		}

		[Test]
		public void CharacterWithEnoughDefense_TakesZeroDamage ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = state.WithEnemies (Generator.CreateCharacter (new Point (2, 2), new Health (10, 10), new Defense (10)).Yield ().ToImmutableList ());

			Character enemy = state.Enemies [0];
			state = Combat.Damage (state, enemy, 5);
			enemy = state.UpdateCharacterReference (enemy);

			Assert.AreEqual (enemy.Health.Current, 10);
		}

		[Test]
		public void KilledPlayer_InvokesDeathAnimation ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = Combat.Damage (state, state.Player, 10);
			Assert.NotNull (PlayerDeathState);
			Assert.Less (PlayerDeathState.Player.Health.Current, 0);
		}

		[Test]
		public void KilledEnemy_IsRemoved ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = state.WithEnemies (Generator.CreateCharacter (new Point (2, 2), new Health (10, 10), new Defense (0)).Yield ().ToImmutableList ());

			state = Combat.Damage (state, state.Enemies [0], 10);
			Assert.Zero (state.Enemies.Count);
			Assert.Null (PlayerDeathState);
		}
	}
}
