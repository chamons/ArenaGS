using System;
using ArenaGS.Model;
using ArenaGS.Utilities;
using ArenaGS.Engine;
using ArenaGS.Platform;
using System.Collections.Immutable;

namespace ArenaGS
{
	public class GameEngine
	{
		public GameState CurrentState { get; private set; }
		IPhysics Physics;
		ISkills Skills;
		public QueryGameState QueryGameState { get; }

		public GameEngine (IFileStorage storage)
		{	
			Dependencies.RegisterInstance <IFileStorage> (storage);
			Dependencies.Register<IActorBehavior> (typeof (DefaultActorBehavior));
			Dependencies.Register<IWorldGenerator> (typeof (WorldGenerator));
			Dependencies.Register<IPhysics> (typeof (Physics));
			Dependencies.Register<ISkills> (typeof (Skills));

			Physics = Dependencies.Get<IPhysics> ();
			Skills = Dependencies.Get<ISkills> ();
			QueryGameState = new QueryGameState ();
		}		

		public void Load ()
		{
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
					break;
				}
				case Command.Wait:
				{
					SetNewState (Physics.WaitPlayer (CurrentState));
					break;
				}
				case Command.Skill:
				{
					SkillTarget target = (SkillTarget)data;
					SetNewState (Skills.Invoke (CurrentState, CurrentState.Player, CurrentState.Player.Skills[target.Index], target.Position));
					break;
				}
				default:
					throw new NotImplementedException ($"Command {c} not implemented.");
			}
			SetNewState (Time.ProcessUntilPlayerReady (CurrentState));
		}
	}
}
