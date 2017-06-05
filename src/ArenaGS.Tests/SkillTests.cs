using System;
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
	[TestFixture]
	class SkillTests
	{
		public class TestPhysics : IPhysics
		{
			public GameState MovePlayer (GameState state, Direction direction) => throw new NotImplementedException ();
			public GameState MoveEnemy (GameState state, Character enemy, Direction direction) => throw new NotImplementedException ();
			public GameState WaitEnemy (GameState state, Character enemy) => throw new NotImplementedException ();
			public Character Wait (Character c) => throw new NotImplementedException ();
			public bool CouldCharacterWalk (GameState state, Character actor, Point newPosition) => throw new NotImplementedException ();

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

		static Skill TestSkill { get; } = new Skill ("Blast", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 5, 0));

		GameState CreateStateWithTestSkill ()
		{
			GameState state = TestScenes.CreateBoxRoomState (Generator);
			return state.WithPlayer (state.Player.WithSkills (new Skill [] { TestSkill }.ToImmutableList ()));
		}

		[Test]
		public void ActorUsingUnownedSkill_Throws ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = TestScenes.CreateTinyRoomState (Generator);
				Skills.Invoke (state, state.Player, TestSkill, new Point (2, 2));
			});
		}

		[Test]
		public void ActorIsUsingSkillOutOfRange_Throws ()
		{
			Assert.Throws<InvalidOperationException> (() =>
			{
				GameState state = CreateStateWithTestSkill ();
				Skills.Invoke (state, state.Player, TestSkill, new Point (10, 10));
			});
		}

		[Test]
		public void ActorIsUsingPointSkillValid_DoesDamageToTarget ()
		{
			GameState state = CreateStateWithTestSkill ();

			state = Skills.Invoke (state, state.Player, TestSkill, new Point (3, 3));

			Character enemyHit = state.Enemies.First (x => x.Position == new Point (3, 3));
			Assert.AreEqual (1, Physics.CharactersDamaged.Count);
			Assert.AreEqual (enemyHit.ID, Physics.CharactersDamaged [0].Item1.ID);
		}

		[Test]
		public void ActorIsUsingPointSkillValid_DoesDamageToSelf()
		{
			GameState state = CreateStateWithTestSkill ();

			state = Skills.Invoke (state, state.Player, TestSkill, new Point (1, 1));

			Assert.AreEqual (1, Physics.CharactersDamaged.Count);
			Assert.IsTrue (Physics.CharactersDamaged[0].Item1.IsPlayer);
		}
	}
}