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
		readonly Size MapSize = new Size (400, 400);
		MapView Map;

		public CombatView (Point position, Size size) : base (position, size)
		{
			position.Offset (MapOffset);
			Map = new MapView (position, MapSize);
		}

		public override void Draw (SKCanvas canvas, GameState state)
		{
			canvas.DrawRect (VisualRect, new SKPaint () { Color = SKColors.Blue });
			Map.Draw (canvas, state);
		}
	}
}
