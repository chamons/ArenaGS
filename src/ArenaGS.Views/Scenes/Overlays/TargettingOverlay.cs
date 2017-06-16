using System;
using System.Linq;

using ArenaGS.Model;
using ArenaGS.Utilities;
using ArenaGS.Views.Utilities;
using ArenaGS.Views.Views;
using SkiaSharp;

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

		public object InfoTarget => Skill;

		public void ConfigureMap (MapView map)
		{
		}

		public void DisableOverlay (CombatView combatView)
		{
		}

		public void HandleKeyDown (string character)
		{
			if (KeyboardBindings.IsReturn (character))
			{
				Select ();
				return;
			}

			KeyboardBindings.IsDirectionKey (character).MatchSome (d =>
			{
				Move (d);
				return;
			});
		}

		void MoveAndSnapIfNeeded (Point p)
		{
			Direction direction = State.Player.Position.DirectionTo (p);
			switch (Skill.TargetInfo.TargettingStyle)
			{
				case TargettingStyle.Cone:
				case TargettingStyle.Line:
					CurrentTargettedPosition = State.Player.Position.InDirection (direction);
					break;
				default:
					CurrentTargettedPosition = p;
					break;
			}
		}

		void Move (Direction direction)
		{
			MoveTo (CurrentTargettedPosition.InDirection (direction));
		}

		void MoveTo (Point p)
		{
			MoveAndSnapIfNeeded (p);
			Parent.Invalidate ();
		}

		void Select ()
		{
			if (CurrentPositionIsValidTarget)
			{
				Parent.SetDefaultOverlay ();
				OnTargetSelected (CurrentTargettedPosition);
			}
		}

		public void OnPress (SKPointI point)
		{
			HitTestResults hitTest = Parent.HitTestScene (point);
			if (hitTest != null && hitTest.View is MapView)
			{
				Point position = (Point)hitTest.Data;
				if (CurrentTargettedPosition == position)
					Select ();
				else
					MoveTo (position);
			}
		}

		SKColor CursorColor = SKColors.Yellow.WithAlpha (50);
		SKColor InvalidCursorColor = SKColors.Red.WithAlpha (50);

		SKColor ValidTargetColor = SKColors.Yellow.WithAlpha (100);
		SKColor InvalidTargetColor = SKColors.Red.WithAlpha (100);

		public void Draw (MapView map)
		{
			bool validTarget = CurrentPositionIsValidTarget;
			SKColor color = validTarget ? ValidTargetColor : InvalidTargetColor;

			if (validTarget)
			{
				foreach (var tile in QueryGameState.AffectedPointsForSkill (State, Skill, CurrentTargettedPosition))
					map.DrawOverlaySquare (tile, color);
				map.DrawOverlaySquare (CurrentTargettedPosition, CursorColor);
			}
			else
			{
				map.DrawOverlaySquare (CurrentTargettedPosition, InvalidCursorColor);
			}
		}
	}
}