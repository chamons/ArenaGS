using System;
using ArenaGS.Utilities;
using ArenaGS.Views.Views;
using SkiaSharp;

namespace ArenaGS.Views.Scenes.Overlays
{
	class DescribeOverlay : IOverlay
	{
		CombatScene Parent;
		GameController Controller;
		Point CurrentTargettedPosition;

		public DescribeOverlay (CombatScene parent, GameController controller)
		{
			Parent = parent;
			Controller = controller;
			CurrentTargettedPosition = Controller.CurrentState.Player.Position;
		}

		public void ConfigureMap (MapView map)
		{
			map.CenterPosition = CurrentTargettedPosition;
		}

		public void DisableOverlay (CombatView combatView)
		{
		}

		public void Draw (MapView map)
		{
		}

		public void HandleKeyDown (string character)
		{
			switch (character)
			{
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

		void MoveTo (Point position)
		{
			CurrentTargettedPosition = position;
			Parent.Invalidate ();
		}

		public void HandleMouseDown (SKPointI point)
		{
		}

		public void HandleMouseUp (SKPointI point)
		{
			HitTestResults hitTest = Parent.HitTestScene (point);
			if (hitTest != null && hitTest.View is MapView)
				MoveTo ((Point)hitTest.Data);
		}
	}
}
