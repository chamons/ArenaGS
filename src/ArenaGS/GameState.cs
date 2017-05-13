using ArenaGS.Model;

namespace ArenaGS
{
	public class GameState
	{
		public Map Map { get; private set; }

		public GameState (Map map)
		{
			Map = map;
		}	
	}
}
