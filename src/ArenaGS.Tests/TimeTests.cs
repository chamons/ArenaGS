﻿using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

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
	class TimeTests : IActorBehavior, IScriptBehavior
	{
		List<ITimedElement> ElementsThatActed;
		ITime Time;
		IGenerator Generator;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Dependencies.RegisterInstance<IActorBehavior> (this);
			Dependencies.RegisterInstance<IScriptBehavior> (this);

			Time = Dependencies.Get<ITime> ();
			Generator = Dependencies.Get<IGenerator> ();

			ElementsThatActed = new List<ITimedElement> ();
		}

		// Registered for IActorBehavior for all characters
		public GameState Act (GameState state, Character c)
		{
			ElementsThatActed.Add (c);
			return state.WithReplaceEnemy (c.WithCT (0));
		}

		// Registered for IActorBehavior for all characters
		public GameState Act (GameState state, MapScript script)
		{
			ElementsThatActed.Add (script);
			return state.WithReplaceScript (script.WithCT (0));
		}

		GameState CreateTestState (int playerCT, int firstCT, int secondCT)
		{
			Character player = Generator.CreateTestPlayer (new Point (1, 1)).WithCT (playerCT);
			Character firstEnemy = Generator.CreateCharacter ("TestEnemy", new Point (2, 2)).WithCT (firstCT);
			Character secondEnemy = Generator.CreateCharacter ("TestEnemy", new Point (2, 2)).WithCT (secondCT);
			return new GameState (null, player, (new Character [] { firstEnemy, secondEnemy }).ToImmutableList (),
								  ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);
		}

		class TestScript : MapScript
		{
			public TestScript (Point position) : base (100, 100)
			{
			}

			public override MapScript WithAdditionalCT (int additionalCT) => this;
			public override MapScript WithCT (int ct) => this;
		}

		GameState CreateTestStateWithScripts (int playerCT, int firstCT, int secondCT, int scriptCT)
		{
			GameState state = CreateTestState (playerCT, firstCT, secondCT);
			state = state.WithScripts (new MapScript [] { Generator.CreateSpawner (new Point (0, 0), "Enemy").WithCT (scriptCT) }.ToImmutableList ());
			return state;
		}

		[Test]
		public void ProcessingToNextPlayer_WithPlayerNext_ReturnsSameState ()
		{
			GameState state = CreateTestState (100, 50, 20);
			GameState newState = Time.ProcessUntilPlayerReady (state);

			Assert.AreEqual (state.Player.CT, newState.Player.CT);
			Assert.AreEqual (state.Enemies [0].CT, newState.Enemies [0].CT);
			Assert.AreEqual (state.Enemies [1].CT, newState.Enemies [1].CT);
			Assert.AreEqual (0, ElementsThatActed.Count);
		}

		[Test]
		public void ProcessingToNextPlayer_WithOnePlayerFirst_GivesCorrectCTs ()
		{
			GameState state = CreateTestState (50, 100, 20);
			state = Time.ProcessUntilPlayerReady (state);

			Assert.AreEqual (100, state.Player.CT);
			Assert.AreEqual (50, state.Enemies [0].CT);
			Assert.AreEqual (70, state.Enemies [1].CT);
			Assert.AreEqual (1, ElementsThatActed.Count);
		}

		[Test]
		public void ProcessingWithScripts_FiresInExpectedOrder ()
		{
			// Debug test
			GameState state = CreateTestStateWithScripts (80, 70, 60, 90);
			state = Time.ProcessUntilPlayerReady (state);

			Assert.AreEqual (100, state.Player.CT);
			Assert.AreEqual (90, state.Enemies [0].CT);
			Assert.AreEqual (80, state.Enemies [1].CT);
			Assert.AreEqual (10, state.Scripts [0].CT);
		}

		[Test]
		public void CharacterWithSmallCTDifferences_GoInCorrectOrder ()
		{
			// Mapscript has higher CT, it should go before player returned
			GameState state = CreateTestStateWithScripts (90, 0, 0, 91);
			state = Time.ProcessUntilPlayerReady (state);
			Assert.AreEqual (1, ElementsThatActed.Count);
			Assert.IsTrue (ElementsThatActed.First () is MapScript);
		}

		[Test]
		public void WithAllCharactersReady_DontDoubleActiveMonster ()
		{
			GameState state = CreateTestState (100, 100, 100);

			// First player action
			state = state.WithPlayer (state.Player.WithCT (0));
			state = Time.ProcessUntilPlayerReady (state);

			// Each enemy should get 1 action, not 2 each
			Assert.AreEqual (2, ElementsThatActed.Count);
		}
	}
}
