using ArenaGS.Model;
using ArenaGS.Utilities;
using ArenaGS.Views.Utilities;
using ArenaGS.Views.Views;
using SkiaSharp;

namespace ArenaGS.Views.Scenes.Overlays
{
	class CombatDefaultOverlay : IOverlay
	{
		GameEngine Engine;
		GameController Controller;
		CombatScene Parent;

		public CombatDefaultOverlay (CombatScene parent, GameController controller, GameEngine engine)
		{
			Parent = parent;
			Controller = controller;
			Engine = engine;		
		}

		public void HandleKeyDown (string character)
		{
			KeyboardBindings.IsDirectionKey (character).MatchSome (d =>
			{
				Engine.AcceptCommand (Command.PlayerMove, d);
				return;
			});

			switch (character)
			{
				case ".":
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
				case "v":
					Parent.SetOverlay (new DescribeOverlay (Parent, Controller));
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
				if (selectedSkill.ReadyForUse)
				{
					switch (selectedSkill.TargetInfo.TargettingStyle)
					{
						case TargettingStyle.Point:
						case TargettingStyle.Cone:
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
		}

		public void OnPress (SKPointI point)
		{
			HitTestResults hitTest = Parent.HitTestScene (point);
			if (hitTest != null)
			{
				if (hitTest.View is SkillBarView)
				{
					int skill = (int)hitTest.Data;
					RequestSkill (skill);
				}
				else if (hitTest.View is MapView)
				{
					Point position = (Point)hitTest.Data;
					Point playerPosition = Controller.CurrentState.Player.Position;
					if (position == playerPosition)
						Engine.AcceptCommand (Command.Wait, null);
					else if (position == playerPosition.InDirection (Direction.North))
						Engine.AcceptCommand (Command.PlayerMove, Direction.North);
					else if (position == playerPosition.InDirection (Direction.South))
						Engine.AcceptCommand (Command.PlayerMove, Direction.South);
					else if (position == playerPosition.InDirection (Direction.West))
						Engine.AcceptCommand (Command.PlayerMove, Direction.West);
					else if (position == playerPosition.InDirection (Direction.East))
						Engine.AcceptCommand (Command.PlayerMove, Direction.East);
					else if (position == playerPosition.InDirection (Direction.Northwest))
						Engine.AcceptCommand (Command.PlayerMove, Direction.Northwest);
					else if (position == playerPosition.InDirection (Direction.Northeast))
						Engine.AcceptCommand (Command.PlayerMove, Direction.Northeast);
					else if (position == playerPosition.InDirection (Direction.Southwest))
						Engine.AcceptCommand (Command.PlayerMove, Direction.Southwest);
					else if (position == playerPosition.InDirection (Direction.Southeast))
						Engine.AcceptCommand (Command.PlayerMove, Direction.Southeast);
				}
			}
		}

		public void ConfigureMap (MapView map)
		{
		}

		public void DisableOverlay (CombatView combatView)
		{
		}

		public void Draw (MapView map)
		{
		}

		public object InfoTarget => Controller.CurrentState.Player;
	}
}
