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
			public TestScript (int id, int ct) : base (id, ct)
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
			TestScript script = new TestScript (1, 100);
			state = state.WithScripts (new MapScript [] { script }.ToImmutableList ());
			state = behavior.Act (state, script);
			Assert.AreEqual (0, state.Scripts [0].CT);
		}

		[Test]
		public void SpawnerScripts_SpawnCorrectTimeAndPlace ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			state = state.WithEnemies (ImmutableList<Character>.Empty);

			SpawnerScript script = new SpawnerScript (1, 100, new Point (2, 3), 2, 2);
			state = state.WithScripts (script.Yield ().ToImmutableList<MapScript> ());

			// First enemy spawned Cooldown (2) turns away
			ScriptBehavior behavior = new ScriptBehavior ();
			int [] expectedEnemyCount = { 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0 };
			foreach (int expected in expectedEnemyCount)
			{
				// Starting with zero enemies and a spawner that can act
				state = state.WithEnemies (ImmutableList<Character>.Empty);
				state = state.WithReplaceScript (state.Scripts [0].WithCT (100));

				state = behavior.Act (state, state.Scripts [0]);
				Assert.AreEqual (expected, state.Enemies.Count);
			}
		}

		[Test]
		public void ReduceCooldownScript_ReducesCooldownUntilZero_ThenDisappaers ()
		{
			const int Cooldown = 3;
			GameState state = TestScenes.CreateBoxRoomStateWithSkillWithResources (Generator, new SkillResources (-1, -1, Cooldown, Cooldown, false));
			state = state.WithEnemies (ImmutableList<Character>.Empty);
			ReduceCooldownScript script = new ReduceCooldownScript (1, 0, state.Player.ID, state.Player.Skills [0].ID);
			state = state.WithScripts (script.Yield ().ToImmutableList<MapScript> ());

			ScriptBehavior behavior = new ScriptBehavior ();
			for (int i = 0; i < Cooldown; ++i)
			{
				state = state.WithReplaceScript (state.Scripts [0].WithAdditionalCT (100));
				state = behavior.Act (state, state.Scripts [0]);
			}
			Assert.False (state.Player.Skills [0].UnderCooldown);
			Assert.Zero (state.Scripts.Count);
		}

		[Test]
		public void ReduceCooldownScript_WithAmmoRecharge_IncreasesAmmoWhenDone ()
		{
			const int Cooldown = 3;
			GameState state = TestScenes.CreateBoxRoomStateWithSkillWithResources (Generator, new SkillResources (1, 2, Cooldown, Cooldown, true));
			state = state.WithEnemies (ImmutableList<Character>.Empty);
			ReduceCooldownScript script = new ReduceCooldownScript (1, 0, state.Player.ID, state.Player.Skills [0].ID);
			state = state.WithScripts (script.Yield ().ToImmutableList<MapScript> ());

			ScriptBehavior behavior = new ScriptBehavior ();
			for (int i = 0; i < Cooldown; ++i)
			{
				state = state.WithReplaceScript (state.Scripts [0].WithAdditionalCT (100));
				state = behavior.Act (state, state.Scripts [0]);
			}
			Assert.False (state.Player.Skills [0].UnderCooldown);
			Assert.AreEqual (2, state.Player.Skills [0].Resources.CurrentAmmo);
			Assert.Zero (state.Scripts.Count);
		}

		[Test]
		public void ReduceCooldownScript_WithRemovedCharacter_DoesNothing ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			Skill skill = Generator.CreateSkill ("Skill", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 2), SkillResources.WithRechargingAmmo (3, 2));
			state = state.WithReplaceEnemy (state.Enemies [0].WithSkills (skill.Yield ().ToImmutableList ()));

			ReduceCooldownScript script = new ReduceCooldownScript (1, 100, state.Enemies [0].ID, state.Enemies [0].Skills [0].ID);
			state = state.WithEnemies (ImmutableList<Character>.Empty);

			ScriptBehavior behavior = new ScriptBehavior ();
			behavior.Act (state, script);
		}
	}

	[TestFixture]
	public class ScriptBehaviorTestsWithStubbedPhysics
	{
		IGenerator Generator;
		TestPhysics Physics;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Generator = Dependencies.Get<IGenerator> ();

			Dependencies.Unregister<IPhysics> ();
			Physics = new TestPhysics ();
			Dependencies.RegisterInstance<IPhysics> (Physics);
		}

		[Test]
		public void AreaDamageScript_DamagesJustCharactersInArea ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			AreaDamageScript damageScript = new AreaDamageScript (1, 100, 1, new Point [] { new  Point (1, 1) }.ToImmutableHashSet ());
			state = state.WithScripts (damageScript.Yield ().ToImmutableList <MapScript> ());

			ScriptBehavior behavior = new ScriptBehavior ();

			state = behavior.Act (state, damageScript);
			Assert.AreEqual (1, Physics.CharactersDamaged.Count);
			Assert.IsTrue (Physics.CharactersDamaged[0].Item1.IsPlayer);
		}
	}
}
