using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class CombatView : View
	{
		readonly Point MapOffset = new Point (10, 10);
		readonly Size MapSize = new Size (600, 540);
		MapView Map;

		public CombatView (Point position, Size size) : base (position, size)
		{
			position.Offset (MapOffset);
			Map = new MapView (position, MapSize);
		}

		public override SKSurface Draw (GameState state)
		{
			Canvas.Clear (SKColors.Black);
			Canvas.DrawSurface (Map.Draw (state), 0, 0);
			return Surface;
		}
	}
}
