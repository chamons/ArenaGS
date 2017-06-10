﻿using System;
using System.Collections.Generic;
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
		SKBitmap [] WallBitmaps = new SKBitmap [4];
		SKBitmap [] FloorBitmaps = new SKBitmap [4];
		SKBitmap PlayerBitmap;
		SKBitmap EnemyBitmap;
		SKBitmap ProjectileBitmap;
		SKBitmap ExplosionBitmap;
		SKBitmap StatueBitmap;
		SKBitmap WaterBitmap;

		IScene Parent;
		AnimationHelper AnimationHelper = new AnimationHelper ();
		AnimationInfo currentAnimation;
		public Point CenterPosition { get; set; }
		MapTileVariants TilesVariant = new MapTileVariants ();   

		public MapView (IScene parent, Point position, Size size) : base (position, size)
		{
			Parent = parent;

			for (int i = 0 ; i < 4; ++i)
				WallBitmaps[i] = Resources.Get ($"stone_gray{i}.png");

			for (int i = 0 ; i < 4; ++i)
				FloorBitmaps[i] = Resources.Get ($"mud{i}.png");

			PlayerBitmap = Resources.Get ("orc_knight.png");
			EnemyBitmap = Resources.Get ("skeletal_warrior.png");
			ProjectileBitmap = Resources.Get ("sling_bullet0.png");
			ExplosionBitmap = Resources.Get ("cloud_fire2.png");
			StatueBitmap = Resources.Get ("crumbled_column_1.png");
			WaterBitmap = Resources.Get ("shallow_water.png");
		}

		public override SKSurface Draw (GameState state)
		{
			BlankCanvas ();
			GameState = state;
			CenterPosition = state.Player.Position;

			Parent.Overlay.ConfigureMap (this);

			var characterToAnimate = CharacterToAnimate ();

			for (int i = 0; i < MapSizeX; ++i)
			{
				for (int j = 0; j < MapSizeY; ++j)
				{
					Point currentUIPosition = new Point (i, j);
					if (IsOnMap (currentUIPosition))
					{
						Point currentModelPosition = TranslateUIToModelPosition (currentUIPosition);
						DrawMapTile (currentUIPosition, currentModelPosition);
					}
				}
			}
			foreach (var enemy in GameState.Enemies.Where (x => x.ID != characterToAnimate?.Item1))
				DrawTile (TranslateModelToUIPosition (enemy.Position), EnemyBitmap);

			if (characterToAnimate != null)
				DrawFloatingTile (TranslateFloatingModelToUIPosition (characterToAnimate.Item2), EnemyBitmap);

			DrawTile (TranslateModelToUIPosition (GameState.Player.Position), PlayerBitmap);

			DrawProjectile ();
			DrawExplosion ();

			Parent.Overlay.Draw (this);

			return Surface;
		}

		void DrawMapTile (Point currentUIPosition, Point currentModelPosition)
		{
			var currentTile = CurrentMap [currentModelPosition];
			int tileIndex = TilesVariant.Get (GameState.Map, currentModelPosition);
			switch (currentTile.Terrain)
			{
				case TerrainType.Floor:
					DrawTile (currentUIPosition, FloorBitmaps [tileIndex]);
					return;
				case TerrainType.Wall:
					DrawTile (currentUIPosition, WallBitmaps [tileIndex]);
					return;
				case TerrainType.Decoration:
					DrawTile (currentUIPosition, WaterBitmap);
					return;
				case TerrainType.DecorationSpecial:
					DrawTile (currentUIPosition, FloorBitmaps [tileIndex]);
					DrawTile (currentUIPosition, StatueBitmap);
					return;
				default:
					throw new NotImplementedException ();	
			}
		}

		void DrawProjectile ()
		{
			if (currentAnimation != null && currentAnimation.Type == AnimationType.Projectile)
			{
				ProjectileAnimationInfo projectileInfo = (ProjectileAnimationInfo)currentAnimation;
				int currentTileIndex = AnimationHelper.Frame / ProjectileTravelTime;
				Point projectilePosition = projectileInfo.Path[currentTileIndex];
				DrawTile (TranslateModelToUIPosition (projectilePosition), ProjectileBitmap);
			}
		}

		void DrawExplosion ()
		{
			if (currentAnimation != null && currentAnimation.Type == AnimationType.Explosion)
			{
				ExplosionAnimationInfo explosionInfo = (ExplosionAnimationInfo)currentAnimation;
				int currentRange = AnimationHelper.Frame / ExplosionExpandTime;
				foreach (var point in explosionInfo.Center.PointsInBurst (currentRange))
					DrawTile (TranslateModelToUIPosition (point), ExplosionBitmap);
			}
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

		public void DrawOverlaySquare (Point mapPosition, SKColor color)
		{
			Point uiPosition = TranslateModelToUIPosition (mapPosition);
			if (IsUIDrawnTile (uiPosition))
				Canvas.DrawRect (DrawRectForUIPosition (uiPosition), new SKPaint () { Color = color });
		}

		void DrawFloatingTile (SKPoint currentUIPosition, SKBitmap image)
		{
			if (IsUIDrawnTile (currentUIPosition))
				Canvas.DrawBitmap (image, DrawRectForFloatingUIPosition (currentUIPosition));
		}

		public void DrawTile (Point currentUIPosition, SKBitmap image)
		{
			if (IsUIDrawnTile (currentUIPosition))
				Canvas.DrawBitmap (image, DrawRectForUIPosition (currentUIPosition));
		}

		SKRect DrawRectForFloatingUIPosition (SKPoint currentUIPosition)
		{
			float left = Position.X + (currentUIPosition.X * MapTileSize);
			float top = Position.Y + (currentUIPosition.Y * MapTileSize);
			var drawPosition = new SKRect (left, top, left + MapTileSize, top + MapTileSize);
			return drawPosition;
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

		public const int MapTileSize = 32;
		public const int MapSizeX = 17;
		public const int MapSizeY = 15;
		public const int MapCenterX = (int)(((MapSizeX - 1) / 2));
		public const int MapCenterY = (int)(((MapSizeY - 1) / 2));

		bool IsUIDrawnTile (Point uiPosition)
		{
			return uiPosition.X >= 0 && uiPosition.X < MapSizeX && uiPosition.Y >= 0 && uiPosition.Y < MapSizeY;
		}

		bool IsUIDrawnTile (SKPoint uiPosition)
		{
			Point convertedPoint = new Point ((int)Math.Ceiling (uiPosition.X), (int)Math.Ceiling (uiPosition.Y));
			return IsUIDrawnTile (convertedPoint);
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
			AnimationHelper.AnimationLoop (CalculateAnimationFrameLength (info), Parent.Invalidate, () =>
			{
				currentAnimation = null;
				AnimationHelper.Reset();
				onAnimationComplete();
			});
#pragma warning restore CS4014
		}

		const int MovementAnimationTime = 2;
		const int ExplosionExpandTime = 2;
		const int ProjectileTravelTime = 2;
		int CalculateAnimationFrameLength (AnimationInfo info)
		{
			switch (info.Type)
			{
				case AnimationType.Movement:
					return MovementAnimationTime;
				case AnimationType.Explosion:
					ExplosionAnimationInfo explosionInfo = (ExplosionAnimationInfo)info;
					return (ExplosionExpandTime * (explosionInfo.Size + 1)) - 1;
				case AnimationType.Projectile:
					ProjectileAnimationInfo projectileInfo = (ProjectileAnimationInfo)info;
					return (ProjectileTravelTime * projectileInfo.Path.Count) - 1;
				default:
					throw new NotImplementedException ();
			}
		}

		public override HitTestResults HitTest(SKPointI point)
		{
			if (!ScreenRect.Contains (point))
				return null;

			int x = (point.X - Position.X) / MapTileSize;
			int y = (point.Y - Position.Y) / MapTileSize;
			return new HitTestResults(this, TranslateUIToModelPosition (new Point (x, y)));
		}
	}
}
