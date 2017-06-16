using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Tests.Utilities;
using NUnit.Framework;

namespace ArenaGS.Tests
{
	[TestFixture]
	public class CombatTests
	{
		ICombat Combat;
		IGenerator Generator;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Combat = Dependencies.Get<ICombat> ();
			Generator = Dependencies.Get<IGenerator> ();
		}


		[Test] // TODO - https://github.com/chamons/ArenaGS/issues/79
		public void DamagedNonPlayerCharacters_Removed ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			Character enemy = state.Enemies [0];
			state = Combat.Damage (state, enemy, 1);
			Assert.Zero (state.Enemies.Count);
		}

		[Test]  // TODO - https://github.com/chamons/ArenaGS/issues/79
		public void DamagedPlayer_Logs ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			Assert.Zero (state.LogEntries.Count);
			state = Combat.Damage (state, state.Player, 1);
			Assert.AreEqual (1, state.LogEntries.Count);
		}
	}
}
