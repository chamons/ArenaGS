using System;
using ArenaGS.Model;
using ArenaGS.Utilities;
using ArenaGS.Engine;
using ArenaGS.Platform;
using System.Threading.Tasks;

namespace ArenaGS
{
	public class GameEngine
	{
		public GameEngine (IFileStorage storage)
		{	
			Dependencies.Register<IFileStorage> (storage);
		}

		public GameState CurrentState { get; private set; }

		public void Load ()
		{
			SetupDefaultDependencies ();
			CurrentState = CreateNewGameState ();
		}

		public void SaveGame ()
		{
			Serialization.Save (CurrentState);
		}

		public void LoadGame ()
		{
			SetNewState (Serialization.Load ());
		}

		public event EventHandler StateChanged;

		void SetNewState (GameState state)
		{
			CurrentState = state;
			StateChanged?.Invoke (this, EventArgs.Empty);
		}

		GameState CreateNewGameState ()
		{
			IMapGenerator mapGenerator = Dependencies.Get<IWorldGenerator> ().GetMapGenerator ("Simple");
			Map map = mapGenerator.Generate (0);
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
			Dependencies.Register<IWorldGenerator> (new WorldGenerator ());
		}
	}
}
