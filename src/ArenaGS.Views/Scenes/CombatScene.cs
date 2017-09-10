﻿using System;
using System.Collections.Generic;
using System.Linq;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;
using ArenaGS.Views.Scenes.Overlays;
using ArenaGS.Views.Views;
using SkiaSharp;

namespace ArenaGS.Views.Scenes
{
	class CombatScene : IScene
	{
		readonly Point CombatOffset = new Point (0, 0);
		readonly Size CombatSize = new Size (1000, 640);

		GameController Controller;
		CombatView CombatView;
		GameEngine Engine;

		CombatDefaultOverlay DefaultOverlay;
		public IOverlay Overlay { get; private set; }	
		ILogger Log;

		public bool AnimationInProgress { get; private set; }
		public event EventHandler AnimationsComplete;

		public CombatScene (GameController controller, GameEngine engine)
		{
			Controller = controller;
			Engine = engine;
			Log = Dependencies.Get<ILogger> ();

			CombatView = new CombatView (this, CombatOffset, CombatSize);
			DefaultOverlay = new CombatDefaultOverlay (this, Controller, Engine);

			SetDefaultOverlay ();
		}

		public void SetDefaultOverlay ()
		{
			SetOverlay (DefaultOverlay);
		}

		public void SetOverlay (IOverlay overlay)
		{
			Overlay?.BeforeDisabled (CombatView);
			Overlay = overlay;
			Invalidate (); // Nothing happened in GameState but we need to redraw
		}

		public void Invalidate ()
		{
			Controller.Invalidate ();
		}

		public void OnDetailPress (SKPointI point)
		{
			if (AnimationInProgress)
				return;

			OverrideInfoTarget = null;
			HitTestResults hitTest = HitTestScene (point);
			if (hitTest != null)
			{
				if (hitTest.View is MapView)
				{
					OverrideInfoTarget = Controller.CurrentState.AllCharacters.FirstOrDefault (x => x.Position == (Point)hitTest.Data);
				}
				else if (hitTest.View is SkillBarView)
				{
					int skillIndex = (int)hitTest.Data;
					if (skillIndex < Controller.CurrentState.Player.Skills.Count)
						OverrideInfoTarget = Controller.CurrentState.Player.Skills [skillIndex];
				}
			}

			Invalidate ();
		}

		public void OnPress (SKPointI point)
		{
			if (AnimationInProgress)
				return;

			OverrideInfoTarget = null;

			Overlay.OnPress (point);
		}

		string EscapeString = ((char)27).ToString (); // 27 is ESC ascii code. macOS returns this
		public void HandleKeyDown (string character)
		{
			if (AnimationInProgress)
				return;

			OverrideInfoTarget = null;

			if (character == EscapeString || character == "Escape")
			{
				SetDefaultOverlay ();
				return;
			}

			Overlay.HandleKeyDown (character);
		}

		object OverrideInfoTarget;

		public void HandlePaint (SKSurface surface)
		{		
			surface.Canvas.Clear (SKColors.Black);

			CombatView.InfoTarget = OverrideInfoTarget ?? Overlay.InfoTarget;
			surface.Canvas.DrawSurface (CombatView.Draw (Controller.CurrentState), 0, 0);
		}

		public void HandleAnimation (AnimationInfo info)
		{
			AnimationInProgress = true;
			CombatView.BeginAnimation (info, OnAnimationComplete);
		}

		void OnAnimationComplete ()
		{
			AnimationInProgress = false;
			AnimationsComplete?.Invoke (this, EventArgs.Empty);
		}

		public HitTestResults HitTestScene (SKPointI point)
		{
			return CombatView.HitTest (point);
		}

		public void HandlePlayerDeath (GameState state)
		{
			SetOverlay (new DeathOverlay (Engine, this, Controller.GameWindow));
		}

		public void HandleNewRound (NewRoundEventArgs args)
		{
			SetOverlay (new NewRoundOverlay (Engine, this, Controller.GameWindow, args.Round));
		}
	}
}
