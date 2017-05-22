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

		string [] CellLabels = new string [NumberOfSkills] { "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "[", "]", "\\" };
		string [] CellImages = new string [NumberOfSkills] { "burning-dot.png", "burning-meteor.png", "wave-crest.png", "wind-hole.png", "snowing.png", "lightning-branches.png", "blunderbuss.png", "grenade.png", "firework-rocket.png", "materials-science.png", "missile-mech.png", "missile-swarm.png", "ray-gun.png", "sentry-gun.png", "armor-vest.png" };


		public override SKSurface Draw (GameState state)
		{
			using (SKPaint cellText = new SKPaint () { Color = SKColors.White, TextSize = 10, TextAlign = SKTextAlign.Center })
			{
				using (SKPaint cellBorder = new SKPaint () { Color = SKColors.White, StrokeWidth = 2, IsStroke = true })
				{
					for (int i = 0; i < NumberOfSkills; ++i)
					{
						int left = Padding + ((Padding + CellSize) * i);
						int top = Padding;
						int right = left + CellSize + Padding;
						int bottom = top + CellSize + Padding;

						Canvas.DrawRect (new SKRect (left, top, right, bottom), cellBorder);

						Canvas.DrawBitmap (Resources.Get (CellImages[i]), new SKRect (left + Padding, top + Padding, right - Padding, bottom - Padding));

						Canvas.DrawText (CellLabels[i], left + (CellSize / 2), CellSize - Padding + 1, cellText);
					}
				}
			}
			return Surface;
		}
	}
}
