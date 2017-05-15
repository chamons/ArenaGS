using System;
using ArenaGS.Model;
using ArenaGS.Utilities;
using ArenaGS.Engine;

namespace ArenaGS
{
	public class GameEngine
	{
		public GameState CurrentState { get; private set; }

		public void LoadGame ()
		{
			SetupDefaultDependencies ();
			CurrentState = CreateNewGameState ();
		}

		public event EventHandler StateChanged;

		void SetNewState (GameState state)
		{
			CurrentState = state;
			StateChanged?.Invoke (this, EventArgs.Empty);
		}

		GameState CreateNewGameState ()
		{
			IMapGenerator mapGenerator = Dependencies.Get<WorldGenerator> ().GetMapGenerator ("Simple");
			Map map = mapGenerator.Generate ();
			Character player = new Character (new Point (5, 5));
			return new GameState (map, player);
		}

		public void AcceptCommand (Command c, object data)
		{
			switch (c)
			{
				case Command.PlayerMove:
				{
					Direction direction = (Direction)data;
					SetNewState (Physics.Move (CurrentState.Player, direction, CurrentState));
					return;
				}
				default:
					throw new NotImplementedException ($"Command {c} not implemented.");
			}
		}

		void SetupDefaultDependencies ()
		{
			Dependencies.Register<WorldGenerator> (new WorldGenerator ());
		}
	}
}
