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
			GameState newState = Physics.MovePlayer (state, Direction.North);
			Assert.AreEqual (newState.Player.Position, new Point (1, 0), "Walk north to floor works");
			Assert.AreEqual (0, newState.Player.CT);
		}

		[Test]
		public void SimpleMovementIntoWall_DoesNotMove ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();
			Assert.IsFalse (Physics.CouldCharacterWalk (state, state.Player, state.Player.Position.InDirection (Direction.South)));

			GameState newState = Physics.MovePlayer (state, Direction.South);

			Assert.AreEqual (newState.Player.Position, new Point (1, 1), "Walk south into wall fails");
			Assert.AreEqual (100, newState.Player.CT);
		}

		[Test]
		public void SimpleMovementNone_DoesNotMove ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();
			GameState newState = Physics.MovePlayer (state, Direction.None);
			Assert.AreEqual (newState.Player.Position, new Point (1, 1), "Walk none should not move");
			Assert.AreEqual (100, newState.Player.CT);
		}

		[Test]
		public void SimpleMovementOffMap_DoesNotMoveOffMap ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();
			GameState firstState = Physics.MovePlayer (state, Direction.North);
			Assert.AreEqual (0, firstState.Player.CT);

			GameState secondState = Physics.MovePlayer (firstState, Direction.North);
			Assert.AreEqual (secondState.Player.Position, new Point (1, 0), "Walk north should walk to edge of map but no farther");
			Assert.AreEqual (0, secondState.Player.CT);
		}

		[Test]
		public void MovementIntoEnemy_DoesNotMove ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();
			GameState newState = Physics.MovePlayer (state, Direction.Southeast);
			Assert.AreEqual (newState.Player.Position, new Point (1, 1), "Walk into enemy should not move us");
			Assert.AreEqual (100, newState.Player.CT);
		}

		[Test]
		public void MovementByEnemyIntoPlayer_DoesNotMove ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();

			state = state.WithReplaceEnemy (state.Enemies[0].WithCT (100));

			GameState newState = Physics.MoveEnemy (state, state.Enemies[0], Direction.Northwest);
			Assert.AreEqual (newState.Enemies[0].Position, new Point (2, 2), "Walk into player should not move enemy");
			Assert.AreEqual (100, newState.Enemies[0].CT);
		}

		[Test]
		public void MovementByEnemyIntoEmpty_DoesMove ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();

			state = state.WithReplaceEnemy (state.Enemies[0].WithCT (100));

			GameState newState = Physics.MoveEnemy (state, state.Enemies[0], Direction.North);
			Assert.AreEqual (newState.Enemies[0].Position, new Point (2, 1), "Walk into empty should move enemy");
			Assert.AreEqual (0, newState.Enemies[0].CT);
		}

		[Test]
		public void WaitCharacter_ReturnsCTAsExpected ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();
			Assert.AreEqual (100, state.Player.CT);
			Assert.AreEqual (100, state.Enemies[0].CT);

			state = Physics.WaitPlayer (state);
			Assert.AreEqual (0, state.Player.CT);
			Assert.AreEqual (100, state.Enemies[0].CT);

			state = Physics.WaitEnemy (state, state.Enemies[0]);
			Assert.AreEqual (0, state.Player.CT);
			Assert.AreEqual (0, state.Enemies[0].CT);
		}
	}
}
