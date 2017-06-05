using ArenaGS.Engine;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;
using NUnit.Framework;

namespace ArenaGS.Tests
{
	[TestFixture]
	class SaveLoadTests
	{
		IGenerator Generator;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Generator = Dependencies.Get<IGenerator> ();
		}

		[Test]
		public void SaveLoad_SmokeTest ()
		{
			Assert.IsFalse (Serialization.SaveGameExists);

			GameState state = TestScenes.CreateRoomFromMapgen (Generator);

			Serialization.Save (state);
			Assert.IsTrue(Serialization.SaveGameExists);

			GameState newState = Serialization.Load ();
			Assert.AreEqual (state.Player.Position, newState.Player.Position);
			Assert.AreEqual (state.Map.Width, newState.Map.Width);
			Assert.AreEqual (state.Map.Height, newState.Map.Height);
			Assert.AreEqual (state.Enemies.Count, newState.Enemies.Count);
			Assert.AreEqual (state.LogEntries.Count, newState.LogEntries.Count);

			Assert.IsFalse (Serialization.SaveGameExists);
		}
	}
}
