using System;
using ArenaGS.Views.Views;
using SkiaSharp;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Views.Scenes.Overlays
{
	class TargettingOverlay : IOverlay
	{
		CombatScene Parent;
		IQueryGameState QueryGameState;
		GameState State;
		Skill Skill;
		Action<Point> OnTargetSelected;
		Point StartingPosition;

		Point CurrentTargettedPosition;

		bool CurrentPositionIsValidTarget => QueryGameState.IsValidTargetForSkill (State, Skill, CurrentTargettedPosition);

		public TargettingOverlay (CombatScene parent, IQueryGameState queryGameState, GameState state, Skill skill, Point startingPosition, Action <Point> onTargetSelected)
		{
			Parent = parent;
			QueryGameState = queryGameState;
			State = state;
			Skill = skill;
			StartingPosition = startingPosition;
			CurrentTargettedPosition = startingPosition;
			OnTargetSelected = onTargetSelected;
		}

		public 	void ConfigureMap (MapView map)
		{
		}

		public void DisableOverlay (CombatView combatView)
		{
		}

		public void HandleKeyDown (string character)
		{
			switch (character)
			{
				case "\r":
				case "Return":
					if (CurrentPositionIsValidTarget)
					{
						Parent.SetDefaultOverlay ();
						OnTargetSelected (CurrentTargettedPosition);
					}
					return;
				case "Up":
				case "NumPad8":
					CurrentTargettedPosition = CurrentTargettedPosition.InDirection (Direction.North);
					Parent.Invalidate ();
					return;
				case "Down":
				case "NumPad2":
					CurrentTargettedPosition = CurrentTargettedPosition.InDirection (Direction.South);
					Parent.Invalidate ();
					return;
				case "Left":
				case "NumPad4":
					CurrentTargettedPosition = CurrentTargettedPosition.InDirection (Direction.West);
					Parent.Invalidate ();
					return;
				case "Right":
				case "NumPad6":
					CurrentTargettedPosition = CurrentTargettedPosition.InDirection (Direction.East);
					Parent.Invalidate ();
					return;
			}
		}

		public void HandleMouseDown (SKPointI point)
		{
		}

		public void HandleMouseUp (SKPointI point)
		{
		}

		public void Draw (MapView map)
		{
			SKColor color = CurrentPositionIsValidTarget ? SKColors.Yellow.WithAlpha (100) : SKColors.Red.WithAlpha (100);
			foreach (var tile in CurrentTargettedPosition.PointsInBurst (Skill.TargetInfo.Area))
				map.DrawOverlaySquare (tile, color);
		}
	}
}
