using System;
using ArenaGS.Model;

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

		GameState CreateNewGameState ()
		{
			IMapGenerator mapGenerator = Dependencies.Get<WorldGenerator> ().GetMapGenerator ("Simple");
			Map map = mapGenerator.Generate ();
			return new GameState (map);
		}

		void SetupDefaultDependencies ()
		{
			Dependencies.Register<WorldGenerator> (new WorldGenerator ());
		}
	}
}
