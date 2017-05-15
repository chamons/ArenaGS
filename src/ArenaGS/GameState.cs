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

		internal GameState WithNewPlayer (Character player)
		{
			GameState newState = Clone ();
			newState.Player = player;
			return newState;
		}

		GameState Clone ()
		{
			return new GameState (this.Map, this.Player);
		}
	}
}
