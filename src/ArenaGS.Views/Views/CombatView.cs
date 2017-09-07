using System;
using ArenaGS.Utilities;
using ArenaGS.Views.Scenes;
using ArenaGS.Views.Utilities;
using SkiaSharp;
using ArenaGS.Platform;

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

		readonly Point InfoOffset = new Point (560, 4);
		readonly Size InfoSize = new Size (200, 485 - 5);

		MapView Map;
		LogView Log;
		SkillBarView SkillBar;
		InfoView Info;
		ILogger Logger;

		public object InfoTarget { get; set; }

		public CombatView (IScene parent, Point position, Size size) : base (position, size)
		{
			Logger = Dependencies.Get<ILogger> ();

			Map = new MapView (parent, MapOffset, MapSize);
			Log = new LogView (LogOffset, LogSize);
			SkillBar = new SkillBarView (SkillBarOffset, SkillBarSize);
			Info = new InfoView (InfoOffset, InfoSize);
		}

		public override SKSurface Draw (GameState state)
		{
			base.Draw (state);

			Canvas.DrawSurface (Map.Draw (state), MapOffset.X, MapOffset.Y);
			Canvas.DrawSurface (Log.Draw (state), LogOffset.X, LogOffset.Y);
			Canvas.DrawSurface (SkillBar.Draw (state), SkillBarOffset.X, SkillBarOffset.Y);

			Info.Target = InfoTarget;
			Canvas.DrawSurface (Info.Draw (state), InfoOffset.X, InfoOffset.Y);

			return Surface;
		}

		public void BeginAnimation (AnimationInfo info, Action onAnimationComplete)
		{
			Map.BeginAnimation (info, onAnimationComplete);
		}

		public override HitTestResults HitTest (SKPointI point)
		{
			Logger.Log (() => $"Hit Test {point}", LogMask.UI, Servarity.Diagnostic);

			HitTestResults results = Map.HitTest (point);
			if (results != null)
				return results;

			results = Log.HitTest (point);
			if (results != null)
				return results;

			return SkillBar.HitTest (point);
		} 
	}
}
