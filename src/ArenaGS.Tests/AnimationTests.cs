using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Tests.Utilities;
using ArenaGS.Utilities;
using NUnit.Framework;

namespace ArenaGS.Tests
{
	[TestFixture]
	public class AnimationTests : IAnimationRequest
	{
		IPhysics Physics;
		ISkills Skills;
		IGenerator Generator;

		List<AnimationInfo> AnimationRequests;
		public void Request (GameState state, AnimationInfo info)
		{
			AnimationRequests.Add (info);
		}

		public void RequestPlayerDead (GameState state) { }

		[SetUp]
		public void Setup ()
		{
			TestDependencies.SetupTestDependencies ();
			Dependencies.Unregister<IAnimationRequest> ();
			Dependencies.RegisterInstance<IAnimationRequest> (this);

			Physics = Dependencies.Get<IPhysics> ();
			Skills = Dependencies.Get<ISkills> ();
			Generator = Dependencies.Get<IGenerator> ();
			AnimationRequests = new List<AnimationInfo> ();
		}

		[Test]
		public void PlayerMovement_ShouldNotFireMovementAnimation ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			state = state.WithEnemies (ImmutableList<Character>.Empty);

			state = Physics.MovePlayer (state, Direction.North);
			Assert.AreEqual (0, AnimationRequests.Count);
		}

		[Test]
		public void PlayerInvalidMovement_ShouldNotFireMovementAnimation ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);
			state = state.WithEnemies (ImmutableList<Character>.Empty);

			state = Physics.MovePlayer (state, Direction.South);
			Assert.Zero (AnimationRequests.Count);
		}

		[Test]
		public void EnemyMovement_ShouldFireMovementAnimation ()
		{
			GameState state = TestScenes.CreateTinyRoomState (Generator);

			state = Physics.MoveEnemy (state, state.Enemies[0], Direction.North);
			Assert.AreEqual (1, AnimationRequests.Count);
			Assert.AreEqual (AnimationType.Movement, AnimationRequests[0].Type);
		}

		[Test]
		public void TargettedSkill_ShouldFireProjectileAnimation ()
		{
			GameState state = TestScenes.AddTestSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1,2));
			Assert.AreEqual (1, AnimationRequests.Count);
			Assert.AreEqual (AnimationType.Projectile, AnimationRequests[0].Type);
		}

		[Test]
		public void TargettedSkillWithArea_ShouldFireProjectileAndExplosionAnimation ()
		{
			GameState state = TestScenes.AddTestAOESkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1,2));
			Assert.AreEqual (2, AnimationRequests.Count);
			Assert.AreEqual (AnimationType.Projectile, AnimationRequests[0].Type);
			Assert.AreEqual (AnimationType.Explosion, AnimationRequests[1].Type);
		}

		[Test]
		public void ConeSkill_ShouldFireConeAnimation ()
		{
			GameState state = TestScenes.AddTestConeSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 2));
			Assert.AreEqual (1, AnimationRequests.Count);
			Assert.AreEqual (AnimationType.Cone, AnimationRequests [0].Type);
		}

		[Test]
		public void LineSkill_ShouldFireSpecificAreaExplosionAnimation ()
		{
			GameState state = TestScenes.AddTestLineSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 2));
			Assert.AreEqual (1, AnimationRequests.Count);
			Assert.AreEqual (AnimationType.SpecificAreaExplosion, AnimationRequests [0].Type);
		}

		[Test]
		public void ChargeSkill_ShouldFireMovementNotProjectile ()
		{
			GameState state = TestScenes.AddChargeSkill (Generator, TestScenes.CreateBoxRoomState (Generator));
			Skills.Invoke (state, state.Player, state.Player.Skills [0], new Point (1, 3));
			Assert.AreEqual (1, AnimationRequests.Count);
			Assert.AreEqual (AnimationType.Movement, AnimationRequests [0].Type);

		}
	}
}
