using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class CombatView : View
	{
		readonly Point MapOffset = new Point (2, 2);
		readonly Size MapSize = new Size (550, 485);

		readonly Point LogOffset = new Point (2, 485 + 10);
		readonly Size LogSize = new Size (550, 85);

		MapView Map;
		LogView Log;

		public CombatView (Point position, Size size) : base (position, size)
		{
			Map = new MapView (MapOffset, MapSize);
			Log = new LogView (LogOffset, LogSize);
		}

		public override SKSurface Draw (GameState state)
		{
			BlankCanvas ();

			Canvas.DrawSurface (Map.Draw (state), MapOffset.X, MapOffset.Y);
			Canvas.DrawSurface (Log.Draw (state), LogOffset.X, LogOffset.Y);

			return Surface;
		}
	}
}
