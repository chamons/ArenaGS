using System;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;
using NUnit.Framework;

namespace ArenaGS.Tests
{
	[TestFixture]
	public class DijkstraTests
	{
		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
		}

		[Test]
		public void TinyMovementOntoFloor_MovesPlayer ()
		{
			GameState state = TestScenes.CreateTinyRoomState ();
			var pathArray = Dijkstra.CalculateShortestPathArray (state.Map, state.Player.Position);
			Console.WriteLine (state.Map);
			Console.WriteLine (pathArray.ToDebugString ());
		}

		[Test]
		public void SimpleMovementOntoFloor_MovesPlayer ()
		{
			Map map = (new SimpleMapGenerator ()).Generate (0);
			var pathArray = Dijkstra.CalculateShortestPathArray (map, new Point (3, 3) );
			Console.WriteLine (map);
			Console.WriteLine (pathArray.ToDebugString ());
		}
	}
}
