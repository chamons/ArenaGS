using System.Linq;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;
using NUnit.Framework;
using System.Collections.Immutable;

namespace ArenaGS.Tests
{
	[TestFixture]
	class ActorBehaviorTests
	{
		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
		}

		[Test]
		public void DefaultActorBehavior_MovesTowardsPlayer_UnlessNextTo ()
		{
			GameState state = TestScenes.CreateBoxRoomState ();
			DefaultActorBehavior behavior = new DefaultActorBehavior ();
			Character closest = state.Enemies.First (x => x.Position == new Point (3, 3));

			state = behavior.Act (state, closest);
			closest = state.Enemies.First (x => x.ID == closest.ID);
			Assert.AreEqual (state.Enemies.First (x => x.ID == closest.ID).Position, new Point (2, 2));

			state = behavior.Act (state, closest);
			Assert.AreEqual (state.Enemies.First (x => x.ID == closest.ID).Position, new Point (2, 2));
		}

		[Test]
		public void DefaultActorBehavior_BlockedEnemy_Waits ()
		{
			Map map = TestScenes.CreateBoxRoom (5, 5);
			Character player = Character.CreatePlayer (new Point (1, 1));
			// W
			//  P E E
			//  E E E
			//  E E E
			Character[] enemies = new Character[] { Character.Create (new Point (2, 1)), Character.Create (new Point (3,1)), Character.Create (new Point (1, 2)),
				Character.Create (new Point (2,2)), Character.Create (new Point (2, 3)), Character.Create (new Point (1,3)), Character.Create (new Point (2, 3)),
				Character.Create (new Point (3,3)) };
			GameState state = new GameState (map, player, enemies.ToImmutableList (), ImmutableList<string>.Empty);

			Character blockedCharacter = enemies.First (x => x.Position == new Point (3, 3));
			DefaultActorBehavior behavior = new DefaultActorBehavior ();
			state = behavior.Act (state, blockedCharacter);
			blockedCharacter = state.Enemies.First (x => x.ID == blockedCharacter.ID);

			Assert.AreEqual (blockedCharacter.Position, new Point (3, 3));
			Assert.AreEqual (blockedCharacter.CT, 0);
		}

		[Test]
		public void DefaultActorBehavior_MultipleTurnMove_TowardsPlayer ()
		{
			GameState state = TestScenes.CreateBoxRoomState ();
			for (int i = 0; i < 10; ++i)
			{
				state = Physics.WaitPlayer (state);
				state = Time.ProcessUntilPlayerReady (state);
			}
		}
	}
}
