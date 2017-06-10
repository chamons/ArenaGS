﻿using System;
using ArenaGS.Engine;
using ArenaGS.Engine.Behavior;
using ArenaGS.Engine.Generators;
using ArenaGS.Platform;

namespace ArenaGS.Tests.Utilities
{
	class TestAnimation : IAnimationRequest
	{
		public void Request (GameState state, AnimationInfo info)
		{
		}
	}

	class TestLogger : ILogger
	{
		public LogMask DiagnosticMask { get => throw new NotImplementedException (); set => throw new NotImplementedException (); }

		public void Log (string message, LogMask mask, Servarity sevarity = Servarity.Normal)
		{
		}

		public void Log (Func<string> messageProc, LogMask mask, Servarity sevarity = Servarity.Normal)
		{
		}
	}

	static class TestDependencies
	{
		internal static void SetupTestDependencies ()
		{
			Dependencies.Clear ();
			Dependencies.Register<IPhysics> (typeof (Physics));
			Dependencies.Register<ISkills> (typeof (Skills));
			Dependencies.Register<ITime> (typeof (Time));
			Dependencies.Register<IWorldGenerator> (typeof (TestWorldGenerator));
			Dependencies.Register<IFileStorage> (typeof (TestFileStorage));
			Dependencies.Register<IActorBehavior> (typeof (DefaultActorBehavior));
			Dependencies.Register<IScriptBehavior> (typeof (ScriptBehavior));
			Dependencies.Register<IGenerator> (typeof(Generator));
			Dependencies.Register<IAnimationRequest> (typeof(TestAnimation));
			Dependencies.Register<ILogger>(typeof(TestLogger));
		}
	}
}
