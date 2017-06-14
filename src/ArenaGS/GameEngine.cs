using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

using ArenaGS.Engine;
using ArenaGS.Engine.Behavior;
using ArenaGS.Engine.Generators;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;

namespace ArenaGS
{
	public class GameStateChangedEventArgs : EventArgs
	{
		public GameState State { get; }

		public GameStateChangedEventArgs (GameState state)
		{
			State = state;
		}
	}

	public class GameEngine : IAnimationRequest
	{
		internal GameState CurrentState { get; private set; }
		IPhysics Physics;
		ISkills Skills;
		ITime Time;
		IGenerator Generator;
		ILogger Log;

		public QueryGameState QueryGameState { get; }

		public GameEngine (IFileStorage storage)
		{	
			Dependencies.RegisterInstance <IFileStorage> (storage);
			Dependencies.Register<IActorBehavior> (typeof (DefaultActorBehavior));
			Dependencies.Register<IScriptBehavior> (typeof (ScriptBehavior));
			Dependencies.Register<IWorldGenerator> (typeof (WorldGenerator));
			Dependencies.Register<IPhysics> (typeof (Physics));
			Dependencies.Register<ISkills> (typeof (Skills));
			Dependencies.Register<ITime> (typeof (Time));
			Dependencies.Register<IGenerator> (typeof(Generator));
			Dependencies.RegisterInstance<IAnimationRequest> (this);
			Dependencies.Register<ILogger> (typeof(Logger));

			Physics = Dependencies.Get<IPhysics> ();
			Skills = Dependencies.Get<ISkills> ();
			Time = Dependencies.Get<ITime> ();
			Generator = Dependencies.Get<IGenerator> ();
			Log = Dependencies.Get<ILogger> ();
			QueryGameState = new QueryGameState ();
		}		

		public void Load ()
		{
			if (Serialization.SaveGameExists)
				SetNewState (Serialization.Load ());
			else
				SetNewState (CreateNewGameState ());
		}

		public void SaveGame ()
		{
			Serialization.Save (CurrentState);
		}

		public void LoadGame ()
		{
			SetNewState (Serialization.Load ());
		}
	
		public event EventHandler<GameStateChangedEventArgs> StateChanged;

		void SetNewState (GameState state)
		{
			CurrentState = state;
			StateChanged?.Invoke (this, new GameStateChangedEventArgs (CurrentState));
		}

		public event EventHandler<AnimationEventArgs> AnimationRequested;
		public void Request (GameState state, AnimationInfo info)
		{
			AnimationRequested?.Invoke (this, new AnimationEventArgs (state, info));
		}

		GameState CreateNewGameState ()
		{
			IMapGenerator mapGenerator = Dependencies.Get<IWorldGenerator> ().GetMapGenerator ("OpenArenaMap");
			Random r = new Random ();
			int hash = r.Next ();
			GeneratedMapData mapData = mapGenerator.Generate (hash);
			Character player = Generator.CreatePlayer (FindOpenSpot (mapData.Map, new Point (8, 8), Enumerable.Empty<Point>()));

			List<Point> enemyPositions = new List<Point> ();
			for (int i = 0; i < 10; ++i)
			{
				Point position = new Point (r.Next (1, mapData.Map.Width), r.Next (1, mapData.Map.Height));
				Point openSpot = FindOpenSpot (mapData.Map, position, enemyPositions.Concat (player.Position.Yield ()));
				if (openSpot != Point.Invalid)
					enemyPositions.Add (openSpot);
			}

			var enemies = Generator.CreateCharacters (enemyPositions);
			ImmutableList<string> startingLog = ImmutableList.Create<string> ();
#if DEBUG
			startingLog = startingLog.Add ($"Map Hash: {hash}");
#endif
			return new GameState (mapData.Map, player, enemies, mapData.Scripts, startingLog);
		}

		Point FindOpenSpot (Map map, Point target, IEnumerable<Point> pointsToAvoid)
		{
			if (map[target].Terrain == TerrainType.Floor && !pointsToAvoid.Contains (target))
				return target;

			for (int i = 0 ; i < 3 ; ++i)
			{
				foreach (var point in target.PointsInBurst (i))
				{
					if (map.IsOnMap (point) && map [point].Terrain == TerrainType.Floor && !pointsToAvoid.Contains (point))
						return point;
				}
	         }
			return Point.Invalid;
		}

		public void AcceptCommand (Command c, object data)
		{
			try
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
			catch (Exception e)
			{
				Log.Log ($"GameEngine threw exception \"{e.Message}\" with stacktrace:\n {e.StackTrace}. Exiting.", LogMask.Engine, Servarity.Normal);
				throw;
			}
		}
	}
}
