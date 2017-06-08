using System;
using System.Linq;
using ArenaGS.Model;
using ArenaGS.Utilities;
using ArenaGS.Views.Scenes;
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
		IScene Parent;
		AnimationHelper AnimationHelper = new AnimationHelper ();
		AnimationInfo currentAnimation;

		public MapView (IScene parent, Point position, Size size) : base (position, size)
		{
			Parent = parent;
			WallBitmap = Resources.Get ("stone_gray1.png");
			FloorBitmap = Resources.Get ("mud3.png");
			PlayerBitmap = Resources.Get ("orc_knight.png");
			EnemyBitmap = Resources.Get ("skeletal_warrior.png");
		}

		public override SKSurface Draw (GameState state, object data)
		{
			BlankCanvas ();
			GameState = state;

			var characterToAnimate = CharacterToAnimate ();

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
			foreach (var enemy in GameState.Enemies.Where (x => characterToAnimate == null || x.ID != characterToAnimate.Item1))
				DrawTile (TranslateModelToUIPosition (enemy.Position), EnemyBitmap);

			if (characterToAnimate != null)
				DrawFloatingTile (TranslateFloatingModelToUIPosition (characterToAnimate.Item2), EnemyBitmap);

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

		Tuple<int, SKPoint> CharacterToAnimate ()
		{
		    if (currentAnimation != null && currentAnimation.Type == AnimationType.Movement)
			{
				MovementAnimationInfo movementInfo = (MovementAnimationInfo)currentAnimation;
				var animatingCharacter = movementInfo.Character;
				int deltaX = movementInfo.NewPosition.X - animatingCharacter.Position.X;
				int deltaY = movementInfo.NewPosition.Y - animatingCharacter.Position.Y;
				float percentageDone = AnimationHelper.PercentComplete;
				SKPoint animatedPosition = new SKPoint (animatingCharacter.Position.X + deltaX * percentageDone, 
				                                        animatingCharacter.Position.Y + deltaY * percentageDone);
				return new Tuple<int, SKPoint> (animatingCharacter.ID, animatedPosition);
			}
			return null;
		}

		void DrawOverlayMapSquare (Point mapPosition, SKColor color)
		{
			Point uiPosition = TranslateModelToUIPosition (mapPosition);
			if (IsUIDrawnTile (uiPosition))
				Canvas.DrawRect (DrawRectForUIPosition (uiPosition), new SKPaint () { Color = color });
		}

		void DrawFloatingTile (SKPoint currentUIPosition, SKBitmap image)
		{
			Canvas.DrawBitmap (image, DrawRectForFloatingUIPosition (currentUIPosition));
		}

		SKRect DrawRectForFloatingUIPosition (SKPoint currentUIPosition)
		{
			float left = Position.X + (currentUIPosition.X * MapTileSize);
			float top = Position.Y + (currentUIPosition.Y * MapTileSize);
			var drawPosition = new SKRect (left, top, left + MapTileSize, top + MapTileSize);
			return drawPosition;
		}

		void DrawTile (Point currentUIPosition, SKBitmap image)
		{
			Canvas.DrawBitmap (image, DrawRectForUIPosition (currentUIPosition));
		}

		SKRect DrawRectForUIPosition (Point currentUIPosition)
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

		SKPoint TranslateFloatingModelToUIPosition (SKPoint p)
		{
			float centerX = CenterPosition.X - p.X;
			float centerY = CenterPosition.Y - p.Y;
			return new SKPoint (MapCenterX - centerX, MapCenterY - centerY);
		}

		Point TranslateUIToModelPosition (Point p)
		{
			int lowX = CenterPosition.X - MapCenterX;
			int lowY = CenterPosition.Y - MapCenterY;
			return new Point (lowX + p.X, lowY + p.Y);
		}

		public void BeginAnimation (AnimationInfo info, Action onAnimationComplete)
		{
			currentAnimation = info;

#pragma warning disable CS4014 // This is desired behavior, we are using it for timing
			AnimationHelper.AnimationLoop (3, Parent.Invalidate, () =>
			{
				currentAnimation = null;
				AnimationHelper.Reset();
				onAnimationComplete();
			});
#pragma warning restore CS4014
		}
	}
}
