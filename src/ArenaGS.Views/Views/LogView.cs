using System;
using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class LogView : View
	{
		public LogView (Point position, Size size) : base (position, size)
		{
		}

		const int MaxLogShown = 5;
		const int LogBorder = 2;
		const int LogLeftSideBorder = LogBorder + 5;
		const int LogLineHeight = 15;

		public override SKSurface Draw (GameState state)
		{
			BlankCanvas ();

			Canvas.DrawRect (new SKRect (0, 0, VisualRect.Width - LogBorder, VisualRect.Height - LogBorder), new SKPaint () { Color = SKColors.White, IsStroke = true });
			for (int i = 0; i < Math.Min (MaxLogShown, state.LogEntries.Count); ++i)
			{
				SKPoint lineCenter = new SKPoint (VisualRect.Left + LogLeftSideBorder, VisualRect.Top + (LogLineHeight * (i + 1)));
				Canvas.DrawText (state.LogEntries[i], lineCenter.X, lineCenter.Y, new SKPaint () { Color = SKColors.White, TextSize = 12, IsAntialias = true, TextAlign = SKTextAlign.Left });
			}
			return Surface;
		}
	}
}
