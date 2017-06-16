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
		SKColor DarkTile = SKColors.Black.WithAlpha (196);
		SKColor DelayedExplosionTile = SKColors.DarkRed.WithAlpha (196);

		IScene Parent;
		AnimationHelper AnimationHelper = new AnimationHelper ();
		AnimationInfo currentAnimation;
		public Point CenterPosition { get; set; }
		MapThemePainter Painter { get; } = new MapThemePainter ();
		MapVisibility CurrentVisibility;

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
			CurrentVisibility = state.CalculateVisibility (state.Player);

			Parent.Overlay.ConfigureMapForDrawing (this);

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

						if (!CurrentVisibility.IsVisible (currentModelPosition))
							DrawOverlaySquare (currentModelPosition, DarkTile);
					}
				}
			}

			foreach (var enemy in GameState.Enemies.Where (x => x.ID != characterToAnimate?.Item1 && CurrentVisibility.IsVisible (x.Position)))
				DrawTile (TranslateModelToUIPosition (enemy.Position), EnemyBitmap);

			if (characterToAnimate != null)
				DrawFloatingTile (TranslateFloatingModelToUIPosition (characterToAnimate.Item2), EnemyBitmap);

			DrawTile (TranslateModelToUIPosition (GameState.Player.Position), PlayerBitmap);
			DrawDelayedDamageAreas ();

			DrawProjectile ();
			DrawExplosion ();
			DrawSpecificExplosion ();
			DrawCones ();

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
				if (CurrentVisibility.IsVisible (projectilePosition))
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
				{
					if (CurrentVisibility.IsVisible (point))
						DrawTile (TranslateModelToUIPosition (point), ExplosionBitmap);
				}
			}
		}

		void DrawSpecificExplosion ()
		{
			if (currentAnimation != null && currentAnimation.Type == AnimationType.SpecificAreaExplosion)
			{
				SpecificAreaExplosionAnimationInfo explosionInfo = (SpecificAreaExplosionAnimationInfo)currentAnimation;
				foreach (var point in explosionInfo.PointsAffected)
				{
					if (CurrentVisibility.IsVisible (point))
						DrawTile (TranslateModelToUIPosition (point), ExplosionBitmap);
				}
			}
		}

		void DrawCones ()
		{
			if (currentAnimation != null && currentAnimation.Type == AnimationType.Cone)
			{
				ConeAnimationInfo coneInfo = (ConeAnimationInfo)currentAnimation;
				int currentRange = AnimationHelper.Frame / ConeExpandTime;
				foreach (var point in coneInfo.Center.PointsInCone (coneInfo.Direction, currentRange).Where(x => coneInfo.PointsAffected.Contains(x)))
				{
					if (CurrentVisibility.IsVisible (point))
						DrawTile (TranslateModelToUIPosition (point), ExplosionBitmap);
				}
			}
		}

		void DrawDelayedDamageAreas ()
		{
			foreach (AreaDamageScript damageScript in GameState.Scripts.OfType<AreaDamageScript> ())
			{
				foreach (Point point in damageScript.Area)
				{
					if (CurrentVisibility.IsVisible (point))
						DrawOverlaySquare (point, DelayedExplosionTile);
				}
			}			
		}

		Tuple<int, SKPoint> CharacterToAnimate ()
		{
		    if (currentAnimation != null && currentAnimation.Type == AnimationType.Movement)
			{
				MovementAnimationInfo movementInfo = (MovementAnimationInfo)currentAnimation;
				if (!CurrentVisibility.IsVisible (movementInfo.NewPosition))
					return null;

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
		const int ConeExpandTime = 2;

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
				case AnimationType.Cone:
					ConeAnimationInfo coneInfo = (ConeAnimationInfo)info;
					return (ConeExpandTime * coneInfo.Length);
				case AnimationType.SpecificAreaExplosion:
					return ProjectileTravelTime;
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

		public void DrawDeathNotice ()
		{
			Canvas.DrawRect (VisualRect, new SKPaint () { Color = SKColors.Black.WithAlpha (225) });
			Canvas.DrawText ("You died.", VisualRect.Width / 2, VisualRect.Height / 2, new SKPaint () { Color = SKColors.White, TextSize = 20, IsAntialias = true, TextAlign = SKTextAlign.Center });
			Canvas.DrawText ("Press 'q' to quit or 'n' to start a new game.", VisualRect.Width / 2, (VisualRect.Height / 2) + 25, new SKPaint () { Color = SKColors.White, TextSize = 20, IsAntialias = true, TextAlign = SKTextAlign.Center });
			
		}
	}
}
