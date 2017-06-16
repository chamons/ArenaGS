using System;
using System.Linq;
using ArenaGS.Utilities;
using ArenaGS.Views.Utilities;
using ArenaGS.Views.Views;
using SkiaSharp;
using ArenaGS.Model;

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

		public void ConfigureMapForDrawing (MapView map)
		{
			map.CenterPosition = CurrentTargettedPosition;
		}

		public void BeforeDisabled (CombatView combatView)
		{
		}

		SKColor CursorColor = SKColors.Yellow.WithAlpha (100);

		public void Draw (MapView map)
		{
			map.DrawOverlaySquare (CurrentTargettedPosition, CursorColor);
		}

		public void HandleKeyDown (string character)
		{
			KeyboardBindings.IsDirectionKey (character).MatchSome (d =>
			{
				Move (d);
				return;
			});
			switch (character)
			{
				case "v":
					Parent.SetDefaultOverlay ();
					return;
				default:
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

		public void OnPress (SKPointI point)
		{
			HitTestResults hitTest = Parent.HitTestScene (point);
			if (hitTest != null && hitTest.View is MapView)
				MoveTo ((Point)hitTest.Data);
		}

		public object InfoTarget
		{
			get
			{
				Character targettedCharacter = Controller.CurrentState.AllCharacters.FirstOrDefault (x => x.Position == CurrentTargettedPosition);
				if (targettedCharacter != null)
					return targettedCharacter;
				return Controller.CurrentState.Player;
			}
		}
	}
}
