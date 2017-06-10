﻿using System;
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

		public void ConfigureMap (MapView map)
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
					Select ();
					return;
				case "Up":
				case "NumPad8":
					Move (Direction.North);
					return;
				case "Down":
				case "NumPad2":
					Move (Direction.South);
					return;
				case "Left":
				case "NumPad4":
					Move (Direction.West);
					return;
				case "Right":
				case "NumPad6":
					Move (Direction.East);
					return;
			}
		}

		void Move (Direction direction)
		{
			MoveTo (CurrentTargettedPosition.InDirection (direction));
		}

		void MoveTo (Point p)
		{
			CurrentTargettedPosition = p;
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

		public void HandleMouseDown (SKPointI point)
		{
		}

		public void HandleMouseUp (SKPointI point)
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

		public void Draw (MapView map)
		{
			SKColor color = CurrentPositionIsValidTarget ? SKColors.Yellow.WithAlpha (100) : SKColors.Red.WithAlpha (100);
			foreach (var tile in QueryGameState.AffectedPointsForSkill (State, Skill, CurrentTargettedPosition))
				map.DrawOverlaySquare (tile, color);
		}
	}
}