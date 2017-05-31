using System;
using ArenaGS.Views.Views;
using SkiaSharp;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Views.Scenes
{
	class TargettingOverlay : IOverlay
	{
		CombatScene Parent;
		TargettingInfo TargetInfo;
		Point StartingPosition;
		Point CurrentTargettedPosition;
		Action<Point> OnTargetSelected;

		// TODO - This seems wrong
		bool CurrentPositionIsValidTarget => CurrentTargettedPosition.NormalDistance (StartingPosition) <= TargetInfo.Range;

		public TargettingOverlay (CombatScene parent, TargettingInfo info, Point startingPosition, Action <Point> onTargetSelected)
		{
			Parent = parent;
			TargetInfo = info;
			StartingPosition = startingPosition;
			CurrentTargettedPosition = startingPosition;
			OnTargetSelected = onTargetSelected;
		}

		public void ConfigureView (CombatView combatView)
		{
			combatView.SetTargetOverlay (CurrentTargettedPosition, TargetInfo.Area, CurrentPositionIsValidTarget);
		}

		public void DisableOverlay (CombatView combatView)
		{
			combatView.ClearTargetOverlay ();
		}

		public void HandleKeyDown (string character)
		{
			switch (character)
			{
				case "Return": // TODO MAC
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
	}
}
