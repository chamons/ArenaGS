namespace ArenaGS.Utilities
{
	public class MapVisibility
	{
		public int Width { get; }
		public int Height { get; }
		public bool [,] Visibility { get; set; }

		public MapVisibility (int width, int height)
		{
			Width = width;
			Height = height;
			Visibility = new bool [Width, Height];
		}

		public bool IsVisible (Point p) => Visibility [p.X, p.Y];
	}
}
