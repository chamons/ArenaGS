using ArenaGS.Engine;
using ArenaGS.Engine.Utilities;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;

using NUnit.Framework;

namespace ArenaGS.Tests
{
	[TestFixture]
	public class DijkstraTests
	{
		IGenerator Generator;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Generator = Dependencies.Get<IGenerator> ();
		}

		[Test]
		public void TinyRoomDijkstra_CorrectValues ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			var pathArray = Dijkstra.CalculateShortestPathArray (state.Map, state.Player.Position);
			Assert.AreEqual (1, pathArray [0, 0]);
			Assert.AreEqual (1, pathArray [0, 1]);
			Assert.AreEqual (1, pathArray [0, 2]);
			Assert.AreEqual (1, pathArray [1, 0]);
			Assert.AreEqual (0, pathArray [1, 1]);
			Assert.AreEqual (-1, pathArray [1, 2]);
			Assert.AreEqual (1, pathArray [2, 0]);
			Assert.AreEqual (1, pathArray [2, 1]);
			Assert.AreEqual (1, pathArray [2, 2]);

		}

		[Test]
		public void BoxRoomDijkstra_CorrectValues ()
		{
			Map map = TestScenes.CreateBoxRoom (15, 10);
			var pathArray = Dijkstra.CalculateShortestPathArray (map, new Point (3, 3));
			Assert.AreEqual (2, pathArray [1, 1]);
			Assert.AreEqual (-1, pathArray [14, 1]);
			Assert.AreEqual (0, pathArray [3, 3]);
			Assert.AreEqual (10, pathArray [13, 8]);
		}

		[Test]
		public void TinyMazeDijstra_CorrectValues ()
		{
			Map map = TestScenes.CreateTinyMaze ();
			var pathArray = Dijkstra.CalculateShortestPathArray (map, new Point (1, 1));
			Assert.AreEqual (0, pathArray [1, 1]);
			Assert.AreEqual (1, pathArray [1, 2]);
			Assert.AreEqual (2, pathArray [1, 3]);
			Assert.AreEqual (2, pathArray [2, 3]);
			Assert.AreEqual (3, pathArray [3, 3]);
			Assert.AreEqual (3, pathArray [3, 2]);
			Assert.AreEqual (4, pathArray [3, 1]);
		}
	}
}
