using ArenaGS.Model;
using ArenaGS.Utilities;
using ArenaGS.Views.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class MapView : View
	{
		SKBitmap WallBitmap;
		SKBitmap FloorBitmap;
		SKBitmap PlayerBitmap;
		SKBitmap EnemyBitmap;

		public MapView (Point position, Size size) : base (position, size)
		{
			WallBitmap = Resources.Get ("stone_gray1.png");
			FloorBitmap = Resources.Get ("mud3.png");
			PlayerBitmap = Resources.Get ("orc_knight.png");
			EnemyBitmap = Resources.Get ("skeletal_warrior.png");
		}

		public override SKSurface Draw (GameState state, object data)
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
			foreach (var enemy in GameState.Enemies)
				DrawTile (TranslateModelToUIPosition (enemy.Position), EnemyBitmap);

			DrawTile (new Point (MapCenterX, MapCenterY), PlayerBitmap);

			if (data != null)
			{
				TargetOverlayInfo overlayInfo = (TargetOverlayInfo)data;
				SKColor color = overlayInfo.Valid ? SKColors.Yellow.WithAlpha (100) : SKColors.Red.WithAlpha (100);
				foreach (var tile in overlayInfo.Position.PointsInBurst (overlayInfo.Area))
					DrawOverlayMapSquare (tile, color);
			}

			return Surface;
		}

		void DrawOverlayMapSquare (Point mapPosition, SKColor color)
		{
			Point uiPosition = TranslateModelToUIPosition (mapPosition);
			if (IsUIDrawnTile (uiPosition))
				Canvas.DrawRect (DrawRectForUIPosition (uiPosition), new SKPaint () { Color = color });
		}

		void DrawTile (Point currentUIPosition, SKBitmap image)
		{
			Canvas.DrawBitmap (image, DrawRectForUIPosition (currentUIPosition));
		}

		private SKRect DrawRectForUIPosition (Point currentUIPosition)
		{
			int left = Position.X + (currentUIPosition.X * MapTileSize);
			int top = Position.Y + (currentUIPosition.Y * MapTileSize);
			var drawPosition = new SKRect (left, top, left + MapTileSize, top + MapTileSize);
			return drawPosition;
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

		bool IsUIDrawnTile (Point uiPosition)
		{
			return uiPosition.X >= 0 && uiPosition.X < MapSizeX && uiPosition.Y >= 0 && uiPosition.Y < MapSizeY;
		}

		public bool IsOnMap (Point uiPosition)
		{
			Point position = TranslateUIToModelPosition (uiPosition);
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
