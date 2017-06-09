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
			Overlay?.DisableOverlay (CombatView);
			Overlay = overlay;
			Invalidate (); // Nothing happened in GameState but we need to redraw
		}

		public void Invalidate ()
		{
			Controller.Invalidate ();
		}

		public void HandleMouseDown (SKPointI point)
		{
			if (AnimationInProgress)
				return;

			Overlay.HandleMouseDown (point);
		}

		public void HandleMouseUp (SKPointI point)
		{
			if (AnimationInProgress)
				return;

			Overlay.HandleMouseUp (point);
		}

		string EscapeString = ((char)27).ToString (); // 27 is ESC ascii code. macOS returns this
		public void HandleKeyDown (string character)
		{
			if (AnimationInProgress)
				return;

			if (character == EscapeString || character == "Escape")
			{
				SetDefaultOverlay ();
				return;
			}

			Overlay.HandleKeyDown (character);
		}

		public void HandlePaint (SKSurface surface)
		{		
			surface.Canvas.Clear (SKColors.Black);

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
	}
}
