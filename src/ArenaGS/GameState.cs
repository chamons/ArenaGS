using ArenaGS.Model;

namespace ArenaGS
{
	public class GameState
	{
		public Map Map { get; private set; }
		public Character Player { get; private set; }

		public GameState (Map map, Character player)
		{
			Map = map;
			Player = player;
		}

		GameState (GameState original)
		{
			Map = original.Map;
			Player = original.Player;
		}

		internal GameState WithNewPlayer (Character player)
		{
			return new GameState (this) { Player = player };
		}
	}
}
