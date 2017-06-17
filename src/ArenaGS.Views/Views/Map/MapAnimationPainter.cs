using ArenaGS.Utilities;
using ArenaGS.Views.Scenes;
using ArenaGS.Views.Utilities;
using SkiaSharp;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ArenaGS.Views.Views
{
	class MapAnimationPainter
	{
		AnimationInfo currentAnimation;
		AnimationHelper AnimationHelper = new AnimationHelper ();
		SKBitmap ProjectileBitmap;
		SKBitmap ExplosionBitmap;

		internal MapAnimationPainter ()
		{
			ProjectileBitmap = Resources.Get ("sling_bullet0.png");
			ExplosionBitmap = Resources.Get ("cloud_fire2.png");
		}

		internal void Setup (IScene parent, AnimationInfo info, Action onAnimationComplete)
		{
			currentAnimation = info;
#pragma warning disable CS4014 // This is desired behavior, we are using it for timing
			AnimationHelper.AnimationLoop (CalculateAnimationFrameLength (info), parent.Invalidate, () =>
			{
				currentAnimation = null;
				AnimationHelper.Reset ();
				onAnimationComplete ();
			});
#pragma warning restore CS4014
		}

		internal void Draw (MapView view)
		{
			DrawProjectile (view);
			DrawExplosion (view);
			DrawSpecificExplosion (view);
			DrawCones (view);
		}

		void DrawProjectile (MapView view)
		{
			if (currentAnimation != null && currentAnimation.Type == AnimationType.Projectile)
			{
				ProjectileAnimationInfo projectileInfo = (ProjectileAnimationInfo)currentAnimation;
				int currentTileIndex = AnimationHelper.Frame / ProjectileTravelTime;
				Point projectilePosition = projectileInfo.Path [currentTileIndex];
				if (view.CurrentVisibility.IsVisible (projectilePosition))
					view.DrawTile (view.TranslateModelToUIPosition (projectilePosition), ProjectileBitmap);
			}
		}

		void DrawExplosion (MapView view)
		{
			if (currentAnimation != null && currentAnimation.Type == AnimationType.Explosion)
			{
				ExplosionAnimationInfo explosionInfo = (ExplosionAnimationInfo)currentAnimation;
				int currentRange = AnimationHelper.Frame / ExplosionExpandTime;
				foreach (var point in explosionInfo.Center.PointsInBurst (currentRange).Where (x => explosionInfo.PointsAffected.Contains (x)))
				{
					if (view.CurrentVisibility.IsVisible (point))
						view.DrawTile (view.TranslateModelToUIPosition (point), ExplosionBitmap);
				}
			}
		}

		void DrawSpecificExplosion (MapView view)
		{
			if (currentAnimation != null && currentAnimation.Type == AnimationType.SpecificAreaExplosion)
			{
				SpecificAreaExplosionAnimationInfo explosionInfo = (SpecificAreaExplosionAnimationInfo)currentAnimation;
				foreach (var point in explosionInfo.PointsAffected)
				{
					if (view.CurrentVisibility.IsVisible (point))
						view.DrawTile (view.TranslateModelToUIPosition (point), ExplosionBitmap);
				}
			}
		}

		void DrawCones (MapView view)
		{
			if (currentAnimation != null && currentAnimation.Type == AnimationType.Cone)
			{
				ConeAnimationInfo coneInfo = (ConeAnimationInfo)currentAnimation;
				int currentRange = AnimationHelper.Frame / ConeExpandTime;
				foreach (var point in coneInfo.Center.PointsInCone (coneInfo.Direction, currentRange).Where (x => coneInfo.PointsAffected.Contains (x)))
				{
					if (view.CurrentVisibility.IsVisible (point))
						view.DrawTile (view.TranslateModelToUIPosition (point), ExplosionBitmap);
				}
			}
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

		internal Tuple<int, SKPoint> CharacterToAnimate (MapView view)
		{
			if (currentAnimation != null && currentAnimation.Type == AnimationType.Movement)
			{
				MovementAnimationInfo movementInfo = (MovementAnimationInfo)currentAnimation;
				if (!view.CurrentVisibility.IsVisible (movementInfo.NewPosition))
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
	}
}
