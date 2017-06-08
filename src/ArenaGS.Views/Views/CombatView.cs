using System;
using ArenaGS.Utilities;
using ArenaGS.Views.Scenes;
using ArenaGS.Views.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class CombatView : View
	{
		readonly Point MapOffset = new Point (2, 2);
		readonly Size MapSize = new Size (550, 485);

		readonly Point LogOffset = new Point (2, 485 + 10);
		readonly Size LogSize = new Size (550, 85);

		readonly Point SkillBarOffset = new Point (2, 580);
		readonly Size SkillBarSize = new Size (550, 40);

		MapView Map;
		LogView Log;
		SkillBarView SkillBar;
		TargetOverlayInfo CurrentOverlayInfo;

		public CombatView (IScene parent, Point position, Size size) : base (position, size)
		{
			Map = new MapView (parent, MapOffset, MapSize);
			Log = new LogView (LogOffset, LogSize);
			SkillBar = new SkillBarView (SkillBarOffset, SkillBarSize);
		}

		public override SKSurface Draw (GameState state, object data = null)
		{
			BlankCanvas ();

			Canvas.DrawSurface (Map.Draw (state, CurrentOverlayInfo), MapOffset.X, MapOffset.Y);
			Canvas.DrawSurface (Log.Draw (state, null), LogOffset.X, LogOffset.Y);
			Canvas.DrawSurface (SkillBar.Draw (state, null), SkillBarOffset.X, SkillBarOffset.Y);

			return Surface;
		}

		public void SetTargetOverlay (Point position, int area, bool valid)
		{
			CurrentOverlayInfo = new TargetOverlayInfo (position, area, valid);
		}

		public void ClearTargetOverlay ()
		{
			CurrentOverlayInfo = null;
		}

		public void BeginAnimation (AnimationInfo info, Action onAnimationComplete)
		{
			Map.BeginAnimation (info, onAnimationComplete);
		}
	}
}
