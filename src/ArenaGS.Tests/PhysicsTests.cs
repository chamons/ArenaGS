using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;
using NUnit.Framework;

namespace ArenaGS.Tests
{
	[TestFixture]
	public class PhysicsTests
	{
		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
		}

		[Test]
		public void SimpleMovementOntoFloor_MovesPlayer ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();
			GameState newState = Physics.Move (state.Player, Direction.North, state);
			Assert.AreEqual (newState.Player.Position, new Point (1, 0), "Walk north to floor works");
		}

		[Test]
		public void SimpleMovementIntoWall_DoesNotMove ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();
			GameState newState = Physics.Move (state.Player, Direction.South, state);
			Assert.AreEqual (newState.Player.Position, new Point (1, 1), "Walk south into wall fails");
		}

		[Test]
		public void SimpleMovementNone_DoesNotMove ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();
			GameState newState = Physics.Move (state.Player, Direction.None, state);
			Assert.AreEqual (newState.Player.Position, new Point (1, 1), "Walk none should not move");
		}

		[Test]
		public void SimpleMovementOffMap_DoesNotMoveOffMap ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();
			GameState firstState = Physics.Move (state.Player, Direction.North, state);
			GameState secondState = Physics.Move (firstState.Player, Direction.North, firstState);
			Assert.AreEqual (secondState.Player.Position, new Point (1, 0), "Walk north should walk to edge of map but no farther");
		}
	}
}
