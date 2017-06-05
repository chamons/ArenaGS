using System;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;
using NUnit.Framework;

namespace ArenaGS.Tests
{
	[TestFixture]
	public class QueryStateTests
	{
		IGenerator Generator;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Generator = Dependencies.Get<IGenerator> ();
		}

		// Fundamentally QueryState just invokes other APIs in a read-only 
		// Just smoke test connections
		[Test]
		public void QuerySmokeTest ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			Skill skill = new Skill ("Blast", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 5, 0));

			QueryGameState query = new QueryGameState ();
			Assert.IsTrue (query.IsValidTargetForSkill (state, skill, new Point (2, 2)));
			Assert.IsTrue (query.IsValidTargetForSkill (state, skill, new Point (0, 1)));
			Assert.IsFalse (query.IsValidTargetForSkill (state, skill, new Point (-10, 2)));
		}
	}
}
