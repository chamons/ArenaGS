using ArenaGS.Utilities;
using ArenaGS.Views.Utilities;
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
