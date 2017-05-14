using ArenaGS.Model;
using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class MapView : View
	{
		public MapView (Point position, Size size) : base (position, size)
		{

		}

		public override SKSurface Draw (GameState state)
		{
			CurrentMap = state.Map;
			Canvas.DrawRect (VisualRect, new SKPaint () { Color = SKColors.Red });

			for (int i = 0; i < MapSizeX; ++i)
			{
				for (int j = 0; j < MapSizeY; ++j)
				{
					Point currentUIPosition = new Point (i, j);
					if (IsOnMap (currentUIPosition))
					{
						var currentTile = CurrentMap[TranslateUIToModelPosition (currentUIPosition)];
						// This is wrong. We should not be offsetting like this
						int left = Position.X + 2 + i * (MapTileSize + 2);
						int top = Position.Y + 2 + j * (MapTileSize + 2);
						var drawPosition = new SKRect (left, top, left + MapTileSize, top + MapTileSize);

						System.Diagnostics.Debug.WriteLine ($"Drawing ({drawPosition})");

						Canvas.DrawRect (drawPosition, currentTile.Terrain == TerrainType.Floor ? EmptyTile : WallTile);
					}
				}
			}
			return Surface;
		}

		Map CurrentMap;
		Point CenterPosition => new Point (15, 15);
		SKPaint EmptyTile = new SKPaint () { Color = SKColors.Gray };
		SKPaint WallTile = new SKPaint () { Color = SKColors.Yellow };

		public const int MapTileSize = 32;
		public const int MapSizeX = 23;
		public const int MapSizeY = 17;
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
