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
		SKBitmap PlayerBitmap;
		SKBitmap EnemyBitmap;
		SKBitmap ProjectileBitmap;
		SKBitmap ExplosionBitmap;

		IScene Parent;
		AnimationHelper AnimationHelper = new AnimationHelper ();
		AnimationInfo currentAnimation;
		public Point CenterPosition { get; set; }
		MapThemePainter Painter { get; } = new MapThemePainter ();

		public MapView (IScene parent, Point position, Size size) : base (position, size)
		{
			Parent = parent;
			PlayerBitmap = Resources.Get ("orc_knight.png");
			EnemyBitmap = Resources.Get ("skeletal_warrior.png");
			ProjectileBitmap = Resources.Get ("sling_bullet0.png");
			ExplosionBitmap = Resources.Get ("cloud_fire2.png");
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
						Painter.DrawMapTile (this, CurrentMap.Theme, currentUIPosition, currentModelPosition, CurrentMap);
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
				foreach (var point in explosionInfo.Center.PointsInBurst (currentRange).Where (x => explosionInfo.PointsAffected.Contains (x)))
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
