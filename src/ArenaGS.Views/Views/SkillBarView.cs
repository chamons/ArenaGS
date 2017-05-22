using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class SkillBarView : View
	{
		public SkillBarView (Point position, Size size) : base (position, size)
		{
		}

		const int NumberOfSkills = 15;
		const int Padding = 2;
		const int CellSize = 32;
		public override SKSurface Draw (GameState state)
		{
			using (SKPaint cellBorder = new SKPaint () { Color = SKColors.White, StrokeWidth = 2, IsStroke = true })
			{
				for (int i = 0; i < NumberOfSkills; ++i)
				{
					int left = Padding + ((Padding + CellSize) * i);
					Canvas.DrawRect (new SKRect (left, Padding, left + CellSize + Padding, Padding + CellSize + Padding), cellBorder);
				}
			}
			
			return Surface;
		}
	}
}
