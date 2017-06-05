﻿using System.Collections.Generic;
using System.Collections.Immutable;

using ArenaGS.Engine;
using ArenaGS.Engine.Behavior;
using ArenaGS.Model;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;

using NUnit.Framework;

namespace ArenaGS.Tests
{
	[TestFixture]
	class TimeTests : IActorBehavior
	{
		List<Character> CharactersThatActed;
		ITime Time;
		IGenerator Generator;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Dependencies.RegisterInstance<IActorBehavior> (this);
			Time = Dependencies.Get<ITime> ();
			Generator = Dependencies.Get<IGenerator> ();

			CharactersThatActed = new List<Character> ();
		}

		// Registered for IActorBehavior for all characters
		public GameState Act (GameState state, Character c)
		{
			CharactersThatActed.Add (c);
			return state.WithReplaceEnemy (c.WithCT (0));
		}

		GameState CreateTestState (int playerCT, int firstCT, int secondCT)
		{
			Character player = Generator.CreatePlayer (new Point (1, 1)).WithCT (playerCT);
			Character firstEnemy = Generator.CreateCharacter (new Point (2, 2)).WithCT (firstCT);
			Character secondEnemy = Generator.CreateCharacter (new Point (2, 2)).WithCT (secondCT);
			return new GameState (null, player, (new Character[] { firstEnemy, secondEnemy }).ToImmutableList (),
			                      ImmutableList<MapScript>.Empty, ImmutableList<string>.Empty);
		}

		[Test]
		public void ProcessingToNextPlayer_WithPlayerNext_ReturnsSameState ()
		{
			GameState state = CreateTestState (100, 50, 20);
			GameState newState = Time.ProcessUntilPlayerReady (state);

			Assert.AreEqual (state.Player.CT, newState.Player.CT);
			Assert.AreEqual (state.Enemies[0].CT, newState.Enemies[0].CT);
			Assert.AreEqual (state.Enemies[1].CT, newState.Enemies[1].CT);
			Assert.AreEqual (0, CharactersThatActed.Count);
		}

		[Test]
		public void ProcessingToNextPlayer_WithOnePlayerFirst_GivesCorrectCTs ()
		{
			GameState state = CreateTestState (50, 100, 20);
			GameState newState = Time.ProcessUntilPlayerReady (state);

			Assert.AreEqual (100, newState.Player.CT);
			Assert.AreEqual (50, newState.Enemies[0].CT);
			Assert.AreEqual (70, newState.Enemies[1].CT);
			Assert.AreEqual (1, CharactersThatActed.Count);
		}
	}
}
