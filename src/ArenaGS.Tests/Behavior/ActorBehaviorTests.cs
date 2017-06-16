using System.Collections.Immutable;
using System.Linq;

using ArenaGS.Engine;
using ArenaGS.Engine.Behavior;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;

using NUnit.Framework;

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
			Character player = Generator.CreateStubPlayer (new Point (1, 1));
			// W
			//  P E E
			//  E E E
			//  E E E
			var enemies = Generator.CreateStubEnemies (new Point [] { new Point (2, 1), new Point (3, 1), new Point (1, 2),
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
			for (int i = 0; i < 10; ++i)
			{
				state = Physics.WaitPlayer (state);
				state = Time.ProcessUntilPlayerReady (state);
			}
		}
	}
}
