using System;
using ArenaGS.Utilities;
using ArenaGS.Views.Views;
using SkiaSharp;

namespace ArenaGS.Views.Scenes
{
	class DescribeOverlay : IOverlay
	{
		CombatScene Parent;
		Point CurrentTargettedPosition;

		public DescribeOverlay (CombatScene parent, Point playerPosition)
		{
			Parent = parent;
			CurrentTargettedPosition = playerPosition;
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
