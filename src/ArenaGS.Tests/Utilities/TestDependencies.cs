using ArenaGS.Engine;
using ArenaGS.Engine.Behavior;
using ArenaGS.Model;
using ArenaGS.Platform;

namespace ArenaGS.Tests.Utilities
{
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

		}
	}
}
