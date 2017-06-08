using System;
using System.Collections.Generic;
using System.Linq;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;
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

		CombatDefault DefaultOverlay;
		IOverlay Overlay;
		ILogger Log;

		public bool AnimationInProgress { get; private set; }
		public event EventHandler AnimationsComplete;

		public CombatScene (GameController controller, GameEngine engine)
		{
			Controller = controller;
			Engine = engine;
			Log = Dependencies.Get<ILogger> ();

			CombatView = new CombatView (this, CombatOffset, CombatSize);
			DefaultOverlay = new CombatDefault (this, Controller, Engine);

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

			Overlay.ConfigureView (CombatView);
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

	class CombatDefault : IOverlay
	{
		GameEngine Engine;
		GameController Controller;
		CombatScene Parent;

		public CombatDefault (CombatScene parent, GameController controller, GameEngine engine)
		{
			Parent = parent;
			Controller = controller;
			Engine = engine;		
		}

		public void HandleKeyDown (string character)
		{
			switch (character)
			{
				case "OemPeriod":
				case "Decimal":
					Engine.AcceptCommand (Command.Wait, null);
					return;
				case "S":
					Engine.SaveGame ();
					return;
				case "L":
					Engine.LoadGame ();
					return;
				case "Up":
				case "NumPad8":
					Engine.AcceptCommand (Command.PlayerMove, Direction.North);
					return;
				case "Down":
				case "NumPad2":
					Engine.AcceptCommand (Command.PlayerMove, Direction.South);
					return;
				case "Left":
				case "NumPad4":
					Engine.AcceptCommand (Command.PlayerMove, Direction.West);
					return;
				case "Right":
				case "NumPad6":
					Engine.AcceptCommand (Command.PlayerMove, Direction.East);
					return;
				case "1":
				case "D1":
					RequestSkill (0);
					return;
				case "2":
				case "D2":
					RequestSkill (1);
					return;
				case "3":
				case "D3":
					RequestSkill (2);
					return;
				case "4":
				case "D4":
					RequestSkill (3);
					return;
				case "5":
				case "D5":
					RequestSkill (4);
					return;
				case "6":
				case "D6":
					RequestSkill (5);
					return;
				case "7":
				case "D7":
					RequestSkill (6);
					return;
				case "8":
				case "D8":
					RequestSkill (7);
					return;
				case "9":
				case "D9":
					RequestSkill (8);
					return;
				case "0":
				case "D0":
					RequestSkill (9);
					return;
				case "-":
				case "OemMinus":
					RequestSkill (10);
					return;
				case "+":
				case "OemPlus":
					RequestSkill (11);
					return;
				case "\\":
				case "Oem5":
					RequestSkill (12);
					return;
			}
		}

		internal void RequestSkill (int index)
		{
			var state = Controller.CurrentState;

			var skills = state.Player.Skills;
			if (index < skills.Count)
			{
				Skill selectedSkill = skills [index];
				switch (selectedSkill.TargetInfo.TargettingStyle)
				{
					case TargettingStyle.Point:
						TargettingOverlay overlay = new TargettingOverlay (Parent, Engine.QueryGameState, state, selectedSkill, state.Player.Position, p =>
						{
							Engine.AcceptCommand (Command.Skill, new SkillTarget () { Index = index, Position = p });
						});
						Parent.SetOverlay (overlay);
						return;
					case TargettingStyle.None:
						Engine.AcceptCommand (Command.Skill, new SkillTarget () { Index = index });
						return;
				}
			}
		}

		public void HandleMouseDown (SKPointI point)
		{
		}

		public void HandleMouseUp (SKPointI point)
		{
		}

		public void ConfigureView (CombatView combatView)
		{
		}

		public void DisableOverlay (CombatView combatView)
		{
		}
	}
}
