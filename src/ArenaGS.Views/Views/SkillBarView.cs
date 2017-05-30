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
		const bool ShowHotkey = true;

		string [] CellLabels = new string [NumberOfSkills] { "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "[", "]", "\\" };
		string [] CellImages = new string [NumberOfSkills] { "burning-dot.png", "burning-meteor.png", "wave-crest.png", "wind-hole.png", "snowing.png", "lightning-branches.png", "blunderbuss.png", "grenade.png", "firework-rocket.png", "materials-science.png", "missile-mech.png", "missile-swarm.png", "ray-gun.png", "sentry-gun.png", "armor-vest.png" };

		SKPaint BlackPaint = new SKPaint () { Color = SKColors.Black };
		SKPaint AntialiasPaint = new SKPaint () { IsAntialias = false };
		SKPaint CellText = new SKPaint () { Color = SKColors.White, TextSize = 10, TextAlign = SKTextAlign.Center };
		SKPaint CellBorder = new SKPaint () { Color = SKColors.White, StrokeWidth = 2, IsStroke = true };

		public override SKSurface Draw (GameState state)
		{
			for (int i = 0; i < NumberOfSkills; ++i)
			{
				int left = Padding + ((Padding + CellSize) * i);
				int top = Padding;
				int right = left + CellSize + Padding;
				int bottom = top + CellSize + Padding;

				Canvas.DrawRect (new SKRect (left, top, right, bottom), CellBorder);

				Canvas.DrawBitmap (Resources.Get (CellImages[i]), new SKRect (left + Padding, top + Padding, right - Padding, bottom - Padding), AntialiasPaint);

				if (ShowHotkey)
				{
					int textLeft = left + (CellSize / 2);
					int textTop = CellSize + Padding + 3;
					Canvas.DrawRect (new SKRect (textLeft - 3, textTop - 8, textLeft + 4, textTop + 8), BlackPaint);
					Canvas.DrawText (CellLabels [i], textLeft, CellSize + Padding + 3, CellText);
				}
			}

			return Surface;
		}
	}
}
