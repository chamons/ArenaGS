using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Utilities;
using NUnit.Framework;

namespace ArenaGS.Tests
{
	[TestFixture]
	public class PhysicsTests
	{
		private static GameState SetupTestState ()
		{
			var c = new Character (new Point (1, 1));
			var map = new Map (3, 3);
			map.Set (new Point (1, 0), TerrainType.Floor);
			map.Set (new Point (1, 2), TerrainType.Wall);
			return new GameState (map, c);
		}

		[Test]
		public void SimpleMovementOntoFloor_MovesPlayer ()
		{
			GameState state = SetupTestState ();
			GameState newState = Physics.Move (state.Player, Direction.North, state);
			Assert.AreEqual (newState.Player.Position, new Point (1, 0), "Walk north to floor works");
		}

		[Test]
		public void SimpleMovementIntoWall_DoesNotMove ()
		{
			GameState state = SetupTestState ();
			GameState newState = Physics.Move (state.Player, Direction.South, state);
			Assert.AreEqual (newState.Player.Position, new Point (1, 1), "Walk south into wall fails");
		}

		[Test]
		public void SimpleMovementNone_DoesNotMove ()
		{
			GameState state = SetupTestState ();
			GameState newState = Physics.Move (state.Player, Direction.None, state);
			Assert.AreEqual (newState.Player.Position, new Point (1, 1), "Walk none should not move");
		}
	}
}
