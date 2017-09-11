﻿using System;
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
			Dependencies.RegisterInstance<IAnimationRequest> (this);
			Dependencies.RegisterInstance<IFileStorage> (storage);
			Dependencies.Register<IActorBehavior> (typeof (DefaultActorBehavior));
			Dependencies.Register<IScriptBehavior> (typeof (ScriptBehavior));
			Dependencies.Register<IWorldGenerator> (typeof (WorldGenerator));
			Dependencies.Register<IPhysics> (typeof (Physics));
			Dependencies.Register<ISkills> (typeof (Skills));
			Dependencies.Register<ICombat> (typeof (Combat));
			Dependencies.Register<ITime> (typeof (Time));
			Dependencies.Register<IGenerator> (typeof(Generator));
			Dependencies.Register<ILogger> (typeof(Logger));
			Dependencies.Register<IRandomGenerator> (typeof (RandomGenerator));

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
				LoadGame ();
			else
				StartNewGame ();
		}

		public void SaveGame ()
		{
			Serialization.Save (CurrentState);
		}

		public void LoadGame ()
		{
			RequestNewGame ();
			SetNewState (Serialization.Load ());
		}

		void StartNewGame ()
		{
			RequestNewGame ();
			SetNewState (RoundCoordinator.Create (Generator, 1));
		}

		public event EventHandler<GameStateChangedEventArgs> StateChanged;

		void SetNewState (GameState state)
		{
			CurrentState = state;
			StateChanged?.Invoke (this, new GameStateChangedEventArgs (CurrentState));

			if (state.Enemies.Count == 0)
			{
				RequestNewRound (state, state.CurrentRound + 1);
				SetNewState (RoundCoordinator.Create (Generator, state.CurrentRound + 1));
			}
		}

		public event EventHandler<AnimationEventArgs> AnimationRequested;
		public void Request (GameState state, AnimationInfo info)
		{
			AnimationRequested?.Invoke (this, new AnimationEventArgs (state, info));
		}

		public event EventHandler<GameState> PlayerDeath;
		public void RequestPlayerDead (GameState state)
		{
			PlayerDeath?.Invoke (this, state);
		}

		public event EventHandler NewGame;
		public void RequestNewGame ()
		{
			NewGame?.Invoke (this, EventArgs.Empty);
		}

		public event EventHandler<NewRoundEventArgs> NewRound;
		public void RequestNewRound (GameState state, int round)
		{
			NewRound?.Invoke (this, new NewRoundEventArgs (state, round));
		}

		public void AcceptCommand (Command c, object data)
		{
			try
			{
				switch (c)
				{
					case Command.NewGame:
					{
						StartNewGame ();
						break;
					}
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
