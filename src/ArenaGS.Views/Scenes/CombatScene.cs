﻿using System;
using ArenaGS.Model;
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

		public CombatScene (GameController controller, GameEngine engine)
		{
			Controller = controller;
			Engine = engine;
			CombatView = new CombatView (CombatOffset, CombatSize);
			DefaultOverlay = new CombatDefault (this, Engine);

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
			Overlay.HandleMouseDown (point);
		}

		public void HandleMouseUp (SKPointI point)
		{
			Overlay.HandleMouseUp (point);
		}

		string EscapeString = ((char)27).ToString (); // 27 is ESC ascii code. macOS returns this
		public void HandleKeyDown (string character)
		{
			if (character ==  EscapeString || character == "Escape")
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
			surface.Canvas.DrawSurface (CombatView.Draw (Engine.CurrentState), 0, 0);
		}
	}

	class CombatDefault : IOverlay
	{
		GameEngine Engine;
		CombatScene Parent;

		public CombatDefault (CombatScene parent, GameEngine engine)
		{
			Parent = parent;
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
			var skills = Engine.CurrentState.Player.Skills;
			if (index < skills.Count)
			{
				Skill selectedSkill = skills [index];
				switch (selectedSkill.TargetInfo.TargettingStyle)
				{
					case TargettingStyle.Point:
						TargettingOverlay overlay = new TargettingOverlay (Parent, Engine.QueryGameState, Engine.CurrentState, selectedSkill, Engine.CurrentState.Player.Position, p =>
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
