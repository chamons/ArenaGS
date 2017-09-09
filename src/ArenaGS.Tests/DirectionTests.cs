using ArenaGS.Utilities;
using NUnit.Framework;

namespace ArenaGS.Tests.Utilities
{
	[TestFixture]
	class DirectionTests
	{
		[Test]
		public void LatticeDistance_SmokeTests ()
		{
			Assert.AreEqual (0, (new Point (2, 2)).LatticeDistance (new Point (2, 2)));
			Assert.AreEqual (1, (new Point (2, 2)).LatticeDistance (new Point (2, 3)));
			Assert.AreEqual (2, (new Point (2, 2)).LatticeDistance (new Point (3, 3)));
		}

		[Test]
		public void GridDistance_SmokeTests ()
		{
			Assert.AreEqual (0, (new Point (2, 2)).GridDistance (new Point (2, 2)));
			Assert.AreEqual (1, (new Point (2, 2)).GridDistance (new Point (2, 3)));
			Assert.AreEqual (1, (new Point (2, 2)).GridDistance (new Point (3, 3)));
			Assert.AreEqual (2, (new Point (2, 2)).GridDistance (new Point (4, 4)));
			Assert.AreEqual (2, (new Point (2, 2)).GridDistance (new Point (3, 4)));
			Assert.AreEqual (6, (new Point (2, 2)).GridDistance (new Point (2, 8)));
			Assert.AreEqual (6, (new Point (2, 2)).GridDistance (new Point (3, 8)));
		}
	}
}
