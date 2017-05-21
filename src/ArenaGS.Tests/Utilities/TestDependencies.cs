using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Platform;

namespace ArenaGS.Tests.Utilities
{
	static class TestDependencies
	{
		internal static void SetupTestDependencies ()
		{
			Dependencies.Register<IWorldGenerator> (new TestWorldGenerator ());
			Dependencies.Register<IFileStorage> (new TestFileStorage ());
			Dependencies.Register<IActorBehavior> (new DefaultActorBehavior ());
		}
	}
}
