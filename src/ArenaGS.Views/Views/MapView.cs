using ArenaGS.Model;
using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class MapView : View
	{
		SKBitmap WallBitmap;
		SKBitmap FloorBitmap;
		SKBitmap PlayerBitmap;

		public MapView (Point position, Size size) : base (position, size)
		{
			WallBitmap = Resources.Get ("stone_gray1.png");
			FloorBitmap = Resources.Get ("mud3.png");
			PlayerBitmap = Resources.Get ("orc_knight.png");
		}

		public override SKSurface Draw (GameState state)
		{
			BlankCanvas ();

			GameState = state;

			for (int i = 0; i < MapSizeX; ++i)
			{
				for (int j = 0; j < MapSizeY; ++j)
				{
					Point currentUIPosition = new Point (i, j);
					if (IsOnMap (currentUIPosition))
					{
						var currentTile = CurrentMap[TranslateUIToModelPosition (currentUIPosition)];
						DrawTile (currentUIPosition, currentTile.Terrain == TerrainType.Floor ? FloorBitmap : WallBitmap);
					}
				}
			}
			DrawTile (new Point (MapCenterX, MapCenterY), PlayerBitmap);

			return Surface;
		}

		void DrawTile (Point currentUIPosition, SKBitmap image)
		{
			int left = Position.X + (currentUIPosition.X * MapTileSize);
			int top = Position.Y + (currentUIPosition.Y * MapTileSize);
			var drawPosition = new SKRect (left, top, left + MapTileSize, top + MapTileSize);

			Canvas.DrawBitmap (image, drawPosition);
		}

		GameState GameState;
		Map CurrentMap => GameState.Map;
		Character Player => GameState.Player;
		Point CenterPosition => Player.Position;

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
