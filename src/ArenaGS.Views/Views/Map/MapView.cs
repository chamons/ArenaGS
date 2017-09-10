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
		SKColor DarkTile = SKColors.Black.WithAlpha (196);
		SKColor DelayedExplosionTile = SKColors.DarkRed.WithAlpha (64);

		IScene Parent;
		public Point CenterPosition { get; set; }
		MapThemePainter Painter { get; } = new MapThemePainter ();
		internal MapVisibility CurrentVisibility { get; private set; }
		MapAnimationPainter AnimationPainter = new MapAnimationPainter ();
		MapCharacterPainter CharacterPainter = new MapCharacterPainter ();

		public MapView (IScene parent, Point position, Size size) : base (position, size)
		{
			Parent = parent;
		}

		public override SKSurface Draw (GameState state)
		{
			base.Draw (state);

			GameState = state;
			CenterPosition = state.Player.Position;
			CurrentVisibility = state.CalculateVisibility (state.Player);

			Parent.Overlay.ConfigureMapForDrawing (this);

			var characterToAnimate = AnimationPainter.CharacterToAnimate (this);

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
				DrawTile (TranslateModelToUIPosition (enemy.Position), CharacterPainter.GetImage (GameState, enemy));

			if (characterToAnimate != null)
				DrawFloatingTile (TranslateFloatingModelToUIPosition (characterToAnimate.Item2), CharacterPainter.GetImage (GameState, characterToAnimate.Item1));

			if (characterToAnimate == null || characterToAnimate.Item1 != Player.ID)
				DrawTile (TranslateModelToUIPosition (GameState.Player.Position), CharacterPainter.GetImage (GameState, GameState.Player));

			DrawDelayedDamageAreas ();

			AnimationPainter.Draw (this);

			Parent.Overlay.Draw (this);

			return Surface;
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

		public void DrawOverlaySquare (Point mapPosition, SKColor color)
		{
			Point uiPosition = TranslateModelToUIPosition (mapPosition);
			if (IsUIDrawnTile (uiPosition))
				Canvas.DrawRect (DrawRectForUIPosition (uiPosition), new SKPaint () { Color = color });
		}

		void DrawFloatingTile (SKPoint currentUIPosition, IEnumerable<SKBitmap> images)
		{
			if (IsUIDrawnTile (currentUIPosition))
			{
				foreach (SKBitmap image in images)
					Canvas.DrawBitmap (image, DrawRectForFloatingUIPosition (currentUIPosition));
			}
		}

		void DrawTile (Point currentUIPosition, IEnumerable<SKBitmap> images)
		{
			if (IsUIDrawnTile (currentUIPosition))
			{
				foreach (SKBitmap image in images)
					Canvas.DrawBitmap (image, DrawRectForUIPosition (currentUIPosition));
			}
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

		internal Point TranslateModelToUIPosition (Point p)
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

		internal Point TranslateUIToModelPosition (Point p)
		{
			int lowX = CenterPosition.X - MapCenterX;
			int lowY = CenterPosition.Y - MapCenterY;
			return new Point (lowX + p.X, lowY + p.Y);
		}

		public void BeginAnimation (AnimationInfo info, Action onAnimationComplete)
		{
			AnimationPainter.Setup (Parent, info, onAnimationComplete);
		}

		public override HitTestResults HitTest(SKPointI point)
		{
			if (!ScreenRect.Contains (point))
				return null;

			int x = (point.X - Position.X) / MapTileSize;
			int y = (point.Y - Position.Y) / MapTileSize;
			return new HitTestResults(this, TranslateUIToModelPosition (new Point (x, y)));
		}

		void DrawNotificationBackground ()
		{
			Canvas.DrawRect (VisualRect, new SKPaint () { Color = SKColors.Black.WithAlpha (225) });
		}

		public void DrawDeathNotice ()
		{
			DrawNotificationBackground ();
			Canvas.DrawText ("You died.", VisualRect.Width / 2, VisualRect.Height / 2, new SKPaint () { Color = SKColors.White, TextSize = 20, IsAntialias = true, TextAlign = SKTextAlign.Center });
			Canvas.DrawText ("Press 'q' to quit or 'n' to start a new game.", VisualRect.Width / 2, (VisualRect.Height / 2) + 25, new SKPaint () { Color = SKColors.White, TextSize = 20, IsAntialias = true, TextAlign = SKTextAlign.Center });
		}

		public void DrawNewRound (int round)
		{
			DrawNotificationBackground ();
			Canvas.DrawText ($"Starting Round {round}", VisualRect.Width / 2, VisualRect.Height / 2, new SKPaint () { Color = SKColors.White, TextSize = 16, IsAntialias = true, TextAlign = SKTextAlign.Center });
		}
	}
}
