using System;
using System.Collections.Immutable;
using ArenaGS.Engine;
using ArenaGS.Engine.Behavior;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;
using NUnit.Framework;

namespace ArenaGS.Tests
{
	[TestFixture]
	public class ScriptBehaviorTests
	{
		class TestScript : MapScript
		{
			public TestScript (Point position, int id, int ct) : base (position, id, ct)
			{
			}

			protected TestScript (MapScript script) : base (script)
			{
			}

			public override MapScript WithCT (int ct) => new TestScript (this) { CT = ct };
			public override MapScript WithAdditionalCT (int additionalCT) => WithCT (CT + additionalCT);
		}

		IGenerator Generator;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Generator = Dependencies.Get<IGenerator> ();
		}

		[Test]
		public void ScriptsActedUpon_HaveCTReduced ()
		{
			ScriptBehavior behavior = new ScriptBehavior ();
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			TestScript script = new TestScript (Point.Empty, 1, 100);
			state = state.WithScripts (new MapScript [] { script }.ToImmutableList ());
			state = behavior.Act (state, script);
			Assert.AreEqual (0, state.Scripts[0].CT);
		}

		[Test]
		public void SpawnerScripts_SpawnCorrectTimeAndPlace ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			state = state.WithEnemies (ImmutableList<Character>.Empty);

			SpawnerScript script = new SpawnerScript (new Point (2,3), 1, 100, 2, 2);
			state = state.WithScripts (new MapScript [] { script }.ToImmutableList ());

			// First enemy spawned Cooldown (2) turns away
			ScriptBehavior behavior = new ScriptBehavior ();
			int [] expectedEnemyCount = {0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0 };
			foreach (int expected in expectedEnemyCount)
			{
				// Starting with zero enemies and a spawner that can act
				state = state.WithEnemies (ImmutableList<Character>.Empty);
				state = state.WithReplaceScript (state.Scripts[0].WithCT (100));

				state = behavior.Act (state, state.Scripts[0]);
				Assert.AreEqual (expected, state.Enemies.Count);

			}
		}
	}
}
