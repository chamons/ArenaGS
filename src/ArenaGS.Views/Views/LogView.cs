using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class LogView : View
	{
		public LogView (Point position, Size size) : base (position, size)
		{
		}

		// How do we get the log info? Immutable state (that seems silly). Events bubbled down?
		public override SKSurface Draw (GameState state)
		{
			BlankCanvas ();

			Canvas.DrawRect (new SKRect (0, 0, VisualRect.Width - 2, VisualRect.Height - 2), new SKPaint () { Color = SKColors.White, IsStroke = true });

			return Surface;
		}
	}
}
