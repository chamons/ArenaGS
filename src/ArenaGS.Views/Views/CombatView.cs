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

		public CombatView (IScene parent, Point position, Size size) : base (position, size)
		{
			Map = new MapView (parent, MapOffset, MapSize);
			Log = new LogView (LogOffset, LogSize);
			SkillBar = new SkillBarView (SkillBarOffset, SkillBarSize);
		}

		public override SKSurface Draw (GameState state)
		{
			BlankCanvas ();

			Canvas.DrawSurface (Map.Draw (state), MapOffset.X, MapOffset.Y);
			Canvas.DrawSurface (Log.Draw (state), LogOffset.X, LogOffset.Y);
			Canvas.DrawSurface (SkillBar.Draw (state), SkillBarOffset.X, SkillBarOffset.Y);

			return Surface;
		}

		public void BeginAnimation (AnimationInfo info, Action onAnimationComplete)
		{
			Map.BeginAnimation (info, onAnimationComplete);
		}
	}
}
