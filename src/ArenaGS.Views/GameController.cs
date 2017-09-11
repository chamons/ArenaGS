﻿using System;
using System.Collections.Generic;
using System.Linq;
using ArenaGS.Platform;
using ArenaGS.Views;
using ArenaGS.Views.Scenes;

using Optional;

namespace ArenaGS
{
	public class QueuedUpdate
	{
		public GameState State { get; }
		public Option<AnimationInfo> Animation { get; }
		public QueuedUpdate (GameState state, AnimationInfo info)
		{
			State = state;
			Animation = info.SomeNotNull ();
		}
	}

	public class GameController
	{
		public IGameWindow GameWindow { get; }
		GameEngine GameEngine;
		IScene CurrentScene;
		ILogger Log;

		public GameState CurrentState { get; private set; }

		Queue <QueuedUpdate> QueuedStates = new Queue<QueuedUpdate> ();
		bool QueuedDeath;
		bool QueuedNewRound;
		int QueuedRoundNumber;

		public GameController (IGameWindow gameWindow)
		{
			GameWindow = gameWindow;
			GameWindow.OnPaint += OnPaint;
			GameWindow.OnPress += OnPress;
			GameWindow.OnDetailPress += OnDetailPress;
			GameWindow.OnKeyDown += OnKeyDown;
			GameWindow.OnQuit += OnQuit;
		}

		public void Startup (IFileStorage storage)
		{
			Resources.LoadResouces ();

			GameEngine = new GameEngine (storage);
			Log = Dependencies.Get <ILogger>();

			// TODO - This will someday need to be calculated based on current GameState
			CurrentScene = new CombatScene (this, GameEngine);
			CurrentScene.AnimationsComplete += OnAnimationComplete;

			GameEngine.StateChanged += OnGameEngineStateChanged;
			GameEngine.AnimationRequested += OnGameAnimation;
			GameEngine.PlayerDeath += OnPlayerDead;
			GameEngine.NewRound += OnNewRound;
			GameEngine.NewGame += OnNewGame;
			GameEngine.Load ();
		}

		private void OnNewGame (object sender, EventArgs e)
		{
			QueuedDeath = false;
			QueuedNewRound = false;
			QueuedRoundNumber = 0;
		}

		void OnNewRound (object sender, NewRoundEventArgs args)
		{
			if (CurrentScene.AnimationInProgress)
			{
				QueuedNewRound = true;
				QueuedRoundNumber = args.Round;
			}
			else
			{
				CurrentScene.HandleNewRound (args.Round);
			}
		}

		void OnPlayerDead (object sender, GameState e)
		{
			if (CurrentScene.AnimationInProgress)
				QueuedDeath = true;
			else
				CurrentScene.HandlePlayerDeath ();
		}

		void OnGameEngineStateChanged (object sender, GameStateChangedEventArgs e)
		{
			if (CurrentScene.AnimationInProgress)
			{
				Log.Log ("OnGameEngineStateChanged - Enqueue.", LogMask.Animation);
				QueuedStates.Enqueue (new QueuedUpdate (e.State, null));
			}
			else
			{
				Log.Log ("OnGameEngineStateChanged - Direct", LogMask.Animation);
				CurrentState = e.State;
				Invalidate ();
			}
		}

		void OnAnimationComplete (object sender, EventArgs e)
		{
			QueuedUpdate nextState = QueuedStates.Dequeue (); 
			Log.Log ($"OnAnimationComplete Count: {QueuedStates.Count} - IsNextAnimation: {nextState.Animation.HasValue}", LogMask.Animation);

			HandleNextQueuedState (nextState);

			Invalidate ();
		}

		void HandleNextQueuedState (QueuedUpdate nextState)
		{
			CurrentState = nextState.State;

			nextState.Animation.Match(animation => {
				Log.Log ($"OnAnimationComplete Starting new animation - {animation.Type}", LogMask.Animation);
				CurrentScene.HandleAnimation (animation);
			},
			() => {
				Log.Log ($"OnAnimationComplete Next state has no animation.", LogMask.Animation);
				if (QueuedStates.Count > 0)
				{
					Log.Log ($"OnAnimationComplete It has more queued states to process.", LogMask.Animation);
					HandleNextQueuedState (QueuedStates.Dequeue ());
				}
				else
				{
					Log.Log ($"OnAnimationComplete It has no queued states.", LogMask.Animation);

					if (QueuedDeath)
					{
						QueuedDeath = false;
						CurrentScene.HandlePlayerDeath ();
					}
					else if (QueuedNewRound)
					{
						QueuedNewRound = false;
						CurrentScene.HandleNewRound (QueuedRoundNumber);
					}
				}
			});
		}

		void OnGameAnimation (object sender, AnimationEventArgs e)
		{
			if (CurrentScene.AnimationInProgress)
			{
				Log.Log ($"OnGameAnimation - Queueing {e.Info.Type}", LogMask.Animation);
				QueuedStates.Enqueue (new QueuedUpdate(e.State, e.Info));
			}
			else
			{
				Log.Log ($"OnGameAnimation - Running {e.Info.Type}", LogMask.Animation);
				CurrentScene.HandleAnimation (e.Info);
			}
		}

		public void Invalidate ()
		{
			GameWindow.Invalidate ();
		}

		private void OnQuit (object sender, EventArgs e)
		{
#if !DEBUG
			GameEngine.SaveGame ();
#endif
		}

		void OnKeyDown (object sender, KeyEventArgs e)
		{
			CurrentScene.HandleKeyDown (e.Character);
		}

		void OnPress (object sender, ClickEventArgs e)
		{
			CurrentScene.OnPress (e.Position);
		}

		void OnDetailPress (object sender, ClickEventArgs e)
		{
			CurrentScene.OnDetailPress (e.Position);
		}

		void OnPaint (object sender, PaintEventArgs e)
		{
			CurrentScene.HandlePaint (e.Surface);
		}
	}
}