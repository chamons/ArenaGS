using System.IO;
using System.Reflection;
using ArenaGS.Model;
using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class MapView : View
	{
		SKBitmap WallBitmap;
		SKBitmap FloorBitmap;

		public MapView (Point position, Size size) : base (position, size)
		{
			WallBitmap = Resources.Get ("mud3.png");
			FloorBitmap = Resources.Get ("stone_gray1.png");
		}

		public override SKSurface Draw (GameState state)
		{
			BlankCanvas ();

			CurrentMap = state.Map;

			for (int i = 0; i < MapSizeX; ++i)
			{
				for (int j = 0; j < MapSizeY; ++j)
				{
					Point currentUIPosition = new Point (i, j);
					if (IsOnMap (currentUIPosition))
						DrawTile (currentUIPosition);
				}
			}
			return Surface;
		}

		void DrawTile (Point currentUIPosition)
		{
			var currentTile = CurrentMap[TranslateUIToModelPosition (currentUIPosition)];
			int left = Position.X + (currentUIPosition.X * MapTileSize);
			int top = Position.Y + (currentUIPosition.Y * MapTileSize);
			var drawPosition = new SKRect (left, top, left + MapTileSize, top + MapTileSize);
			var border = new SKRect (drawPosition.Left, drawPosition.Top, drawPosition.Right - 1, drawPosition.Bottom - 1);

			if (currentTile.Terrain == TerrainType.Floor)
				Canvas.DrawBitmap (WallBitmap, drawPosition);
			else
				Canvas.DrawBitmap (FloorBitmap, drawPosition);
		}

		Map CurrentMap;
		Point CenterPosition => new Point (15, 15);

		public const int MapTileSize = 32;
		public const int MapSizeX = 17;
		public const int MapSizeY = 15;
		public const int MapCenterX = (int)(((MapSizeX - 1) / 2));
		public const int MapCenterY = (int)(((MapSizeY - 1) / 2));

		public bool IsOnMap (Point p)
		{
			Point position = TranslateUIToModelPosition (p);
			return CurrentMap.IsOnMap (position);
		}

		Point TranslateModelToUIPosition (Point p)
		{
			int centerX = CenterPosition.X - p.X;
			int centerY = CenterPosition.Y - p.Y;
			return new Point (MapCenterX - centerX, MapCenterY - centerY);
		}

		Point TranslateUIToModelPosition (Point p)
		{
			int lowX = CenterPosition.X - MapCenterX;
			int lowY = CenterPosition.Y - MapCenterY;
			return new Point (lowX + p.X, lowY + p.Y);
		}
	}
}
