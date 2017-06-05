﻿using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;
using NUnit.Framework;

namespace ArenaGS.Tests
{
	class SkillTestHelpers
	{
		internal static Skill TestSkill { get; } = new Skill ("Blast", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 5, 0));

		internal static GameState CreateStateWithTestSkill (IGenerator generator)
		{
			return AddTestSkill (TestScenes.CreateBoxRoomState (generator));
		}

		internal static GameState AddTestSkill (GameState state)
		{
			return state.WithPlayer (state.Player.WithSkills (new Skill [] { TestSkill }.ToImmutableList ()));
		}
	}


	[TestFixture]
	class SkillTests
	{
		ISkills Skills;
		IGenerator Generator;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Skills = Dependencies.Get<ISkills> ();
			Generator = Dependencies.Get<IGenerator>();
		}

		[Test]
		public void UseOfSkill_ReducesInvokersCT ()
		{
			Skill testSkill = SkillTestHelpers.TestSkill;

			GameState state = SkillTestHelpers.CreateStateWithTestSkill (Generator);
			Character enemy = state.Enemies.First (x => x.Position == new Point (3, 3));
			state = state.WithReplaceEnemy (enemy.WithSkills (new Skill [] { testSkill }.ToImmutableList ()));
			enemy = state.Enemies.First (x => x.ID == enemy.ID);

			Assert.IsTrue (enemy.CT >= 100);
			state = Skills.Invoke (state, enemy, testSkill, new Point(1,1));
			enemy = state.Enemies.First (x => x.ID == enemy.ID);
			Assert.IsTrue (enemy.CT < 100);

			Assert.IsTrue (state.Player.CT >= 100);
			state = Skills.Invoke (state, state.Player, testSkill, new Point (1, 1));
			Assert.IsTrue (state.Player.CT < 100);
		}


		[Test]
		public void ActorUsingUnownedSkill_Throws ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateTinyRoomState (Generator);
				Skills.Invoke (state, state.Player, SkillTestHelpers.TestSkill, new Point (2, 2));
			});
		}

		[Test]
		public void ActorUsingSkillOutOfRange_Throws ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = SkillTestHelpers.CreateStateWithTestSkill (Generator);
				Skills.Invoke (state, state.Player, SkillTestHelpers.TestSkill, new Point (10, 10));
			});
		}

		[Test]
		public void ActorUsingSkillOffMap_Throws ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = SkillTestHelpers.CreateStateWithTestSkill (Generator);
				Skills.Invoke (state, state.Player, SkillTestHelpers.TestSkill, new Point (-10, 10));
			});
		}

		[Test]
		public void SkillUse_ShouldRespectWallsInWay ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateWallRoomState (Generator);
				Skills.Invoke (state, state.Player, state.Player.Skills[0], new Point (3, 1));
			});
		}
	}

	[TestFixture]
	class SkillTestsWithStubbedPhysics
	{
		public class TestPhysics : IPhysics
		{
			public GameState MovePlayer (GameState state, Direction direction) => throw new NotImplementedException ();
			public GameState MoveEnemy (GameState state, Character enemy, Direction direction) => throw new NotImplementedException ();
			public GameState WaitEnemy (GameState state, Character enemy) => throw new NotImplementedException ();
			public bool CouldCharacterWalk (GameState state, Character actor, Point newPosition) => throw new NotImplementedException ();

			public GameState Wait (GameState state, Character c) => state;
			public GameState WaitPlayer (GameState state) => state;

			public List<Tuple<Character, int>> CharactersDamaged = new List<Tuple<Character, int>> ();
			public GameState Damage (GameState state, Character target, int amount)
			{
				CharactersDamaged.Add (new Tuple<Character, int> (target, amount));
				return state;
			}
		}

		ISkills Skills;
		TestPhysics Physics;
		IGenerator Generator;

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Dependencies.Unregister<IPhysics> ();

			Physics = new TestPhysics ();
			Dependencies.RegisterInstance<IPhysics> (Physics);

			Skills = Dependencies.Get<ISkills> ();
			Generator = Dependencies.Get<IGenerator>();
		}

		[Test]
		public void ActorIsUsingPointSkillValid_DoesDamageToTarget ()
		{
			GameState state = SkillTestHelpers.CreateStateWithTestSkill (Generator);

			state = Skills.Invoke (state, state.Player, SkillTestHelpers.TestSkill, new Point (3, 3));

			Character enemyHit = state.Enemies.First (x => x.Position == new Point (3, 3));
			Assert.AreEqual (1, Physics.CharactersDamaged.Count);
			Assert.AreEqual (enemyHit.ID, Physics.CharactersDamaged [0].Item1.ID);
		}

		[Test]
		public void ActorIsUsingPointSkillValid_DoesDamageToSelf()
		{
			GameState state = SkillTestHelpers.CreateStateWithTestSkill (Generator);

			state = Skills.Invoke (state, state.Player, SkillTestHelpers.TestSkill, new Point (1, 1));

			Assert.AreEqual (1, Physics.CharactersDamaged.Count);
			Assert.IsTrue (Physics.CharactersDamaged[0].Item1.IsPlayer);
		}

		[Test]
		public void SkillsWithAreaAffect_AffectMultipleCharacters ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			var areaSkill = new Skill ("AreaBlast", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 5, 3));
			state = state.WithPlayer (state.Player.WithSkills (new Skill [] { areaSkill }.ToImmutableList ()));

			state = Skills.Invoke (state, state.Player, areaSkill, new Point (2, 2));
			Assert.AreEqual (2, Physics.CharactersDamaged.Count);
		}
	}
}