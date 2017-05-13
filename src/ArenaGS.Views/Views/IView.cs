using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	internal abstract class View
	{
		public Point Position { get; protected set; }
		public Size Size { get; protected set; }
		public SKRect VisualRect => new SKRect (Position.X, Position.Y, Position.X + Size.Width, Position.Y + Size.Height);

		protected View (Point position, Size size)
		{
			Position = position;
			Size = size;
		}

		public abstract void Draw (SKCanvas canvas, GameState state);	
	}
}
