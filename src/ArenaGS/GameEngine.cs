﻿using System;
using ArenaGS.Model;
using ArenaGS.Utilities;
using ArenaGS.Engine;
using ArenaGS.Platform;
using System.Threading.Tasks;
using System.Collections.Immutable;

namespace ArenaGS
{
	public class GameEngine
	{
		public GameEngine (IFileStorage storage)
		{	
			Dependencies.Register<IFileStorage> (storage);
			Dependencies.Register<IActorBehavior> (new DefaultActorBehavior ());
		}

		public GameState CurrentState { get; private set; }

		public void Load ()
		{
			SetupDefaultDependencies ();
			if (Serialization.SaveGameExists)
				CurrentState = Serialization.Load ();
			else
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
			Character player = Character.CreatePlayer (new Point (5, 5));
			var enemies = ImmutableList.Create (new Character[] { Character.Create (new Point (1, 1)), Character.Create (new Point (8,7)) });
			return new GameState (map, player, enemies, ImmutableList<string>.Empty);
		}

		public void AcceptCommand (Command c, object data)
		{
			switch (c)
			{
				case Command.PlayerMove:
				{
					Direction direction = (Direction)data;
					SetNewState (Physics.MovePlayer (CurrentState, direction));
					// TODO - SetNewState (Time.ProcessUntilPlayerReady) ?
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
