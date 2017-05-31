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

		CombatView CombatView;
		GameEngine Engine;

		public CombatScene (GameEngine engine)
		{
			Engine = engine;
			CombatView = new CombatView (CombatOffset, CombatSize);
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
				case "D1":
					HandleSkill (0);
					return;
				case "D2":
					HandleSkill (1);
					return;
				case "D3":
					HandleSkill (2);
					return;
				case "D4":
					HandleSkill (3);
					return;
				case "D5":
					HandleSkill (4);
					return;
				case "D6":
					HandleSkill (5);
					return;
				case "D7":
					HandleSkill (6);
					return;
				case "D8":
					HandleSkill (7);
					return;
				case "D9":
					HandleSkill (8);
					return;
				case "D0":
					HandleSkill (19);
					return;
				case "OemMinus":
					HandleSkill (10);
					return;
				case "OemPlus":
					HandleSkill (11);
					return;
				case "Oem5":
					HandleSkill (12);
					return;
			}
		}

		void HandleSkill (int index)
		{
			var skills = Engine.CurrentState.Player.Skills;
			if (index < skills.Count)
			{
				Skill selectedSkill = skills [index];
				switch (selectedSkill.TargetInfo.TargettingStyle)
				{
					case TargettingStyle.Point:
					case TargettingStyle.None:
						Engine.AcceptCommand (Command.Skill, new SkillTarget () { Index = index } );
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

		public void HandlePaint (SKSurface surface)
		{
			surface.Canvas.Clear (SKColors.Black);
			surface.Canvas.DrawSurface (CombatView.Draw (Engine.CurrentState), 0, 0);
		}


	}
}
