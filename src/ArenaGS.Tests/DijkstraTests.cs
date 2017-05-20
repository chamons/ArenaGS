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
			Assert.AreEqual (1, pathArray[0, 0]);
			Assert.AreEqual (1, pathArray[0, 1]);
			Assert.AreEqual (1, pathArray[0, 2]);
			Assert.AreEqual (1, pathArray[1, 0]);
			Assert.AreEqual (0, pathArray[1, 1]);
			Assert.AreEqual (-1, pathArray[1, 2]);
			Assert.AreEqual (1, pathArray[2, 0]);
			Assert.AreEqual (1, pathArray[2, 1]);
			Assert.AreEqual (1, pathArray[2, 2]);

		}

		[Test]
		public void SimpleMovementOntoFloor_MovesPlayer ()
		{
			Map map = TestScenes.CreateBoxRoom (15, 10);
			var pathArray = Dijkstra.CalculateShortestPathArray (map, new Point (3, 3) );
			Assert.AreEqual (2, pathArray[1, 1]);
			Assert.AreEqual (-1, pathArray[14, 1]);
			Assert.AreEqual (0, pathArray[3, 3]);
			Assert.AreEqual (10, pathArray[13, 8]);
		}
	}
}
