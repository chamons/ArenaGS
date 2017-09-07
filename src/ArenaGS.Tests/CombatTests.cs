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

		public void Request (GameState state, AnimationInfo info) { }

		GameState PlayerDeathState;
		public void RequestPlayerDead (GameState state)
		{
			PlayerDeathState = state;
		}

		public void DamagedCharacter_HasFewerHP ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = state.WithTestEnemy (Generator, new Point (2, 2));

			Character enemy = state.Enemies [0];
			state = Combat.Damage (state, enemy, 1);
			enemy = state.UpdateCharacterReference (enemy);

			Assert.Less (enemy.Health.Current, 10);
		}

		[Test]
		public void CharacterWithEnoughDefense_TakesZeroDamage ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			Character enemy = TestEnemyHelper.CreateTestEnemy (Generator, new Point (2, 2));
			enemy = enemy.WithDefense (new Defense (10)).WithHealth (new Health (10));
			state = state.WithEnemies (enemy.Yield ().ToImmutableList ());

			enemy = state.UpdateCharacterReference (enemy);
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
			state = state.WithTestEnemy (Generator, new Point (2, 2));

			state = Combat.Damage (state, state.Enemies [0], 10);
			Assert.Zero (state.Enemies.Count);
			Assert.Null (PlayerDeathState);
		}

		[Test]
		public void Overheals_WillNotExceedMaxHealth ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			state = state.WithPlayer (state.Player.WithCurrentHealth (state.Player.Health.Current - 2));

			state = Combat.Heal (state, state.Player, 2);
			Assert.AreEqual (state.Player.Health.Maximum, state.Player.Health.Current);
		}
	}
}
