using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class MapView : View
	{
		public MapView (Point position, Size size) : base (position, size)
		{

		}

		public override void Draw (SKCanvas canvas, GameState state)
		{
			canvas.DrawRect (VisualRect, new SKPaint () { Color = SKColors.Red });
		}
	}
}
