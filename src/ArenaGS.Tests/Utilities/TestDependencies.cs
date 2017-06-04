using ArenaGS.Engine;
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
			Dependencies.Register<IWorldGenerator> (typeof (TestWorldGenerator));
			Dependencies.Register<IFileStorage> (typeof (TestFileStorage));
			Dependencies.Register<IActorBehavior> (typeof (DefaultActorBehavior));
		}
	}
}
