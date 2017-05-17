using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Tests.Utilities
{
	static class TestScenes
	{
		internal static GameState CreateTinyRoomState ()
		{
			var character = new Character (new Point (1, 1));
			var map = Dependencies.Get<IWorldGenerator> ().GetMapGenerator ("TinyTest").Generate (0);
			return new GameState (map, character);
		}
	}
}
