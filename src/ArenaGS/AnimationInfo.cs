using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS
{
	public enum AnimationType
	{
		Movement,
		Projectile,
		Explosion,
		SpecificAreaExplosion,
		Cone
	}

	public abstract class AnimationInfo
	{
		protected AnimationInfo (AnimationType type)
		{
			Type = type;
		}
		
		public AnimationType Type { get; }
	}

	public class MovementAnimationInfo : AnimationInfo
	{
		public Character Character { get; }
		public Point NewPosition { get; }

		public MovementAnimationInfo (Character character, Point newPosition) : base (AnimationType. Movement)
		{
			Character = character;
			NewPosition = newPosition;
		}
	}

	public class ProjectileAnimationInfo : AnimationInfo
	{
		public ImmutableList <Point> Path { get; }

		public ProjectileAnimationInfo (List<Point> path) : base (AnimationType.Projectile)
		{
			Path = path.ToImmutableList ();
		}
	}

	public class ExplosionAnimationInfo : AnimationInfo
	{
		public Point Center { get; }
		public int Size { get; }
		public ImmutableHashSet<Point> PointsAffected { get; }

		public ExplosionAnimationInfo (Point center, int size, ImmutableHashSet<Point> pointsAffected) : base (AnimationType.Explosion)
		{
			Center = center;
			Size = size;
			PointsAffected = pointsAffected;
		}
	}

	public class ConeAnimationInfo : AnimationInfo
	{
		public Point Center { get; }
		public Direction Direction { get; }
		public int Length { get; }
		public ImmutableHashSet <Point> PointsAffected { get; }

		public ConeAnimationInfo (Point center, Direction direction, int length, ImmutableHashSet<Point> pointsAffected) : base (AnimationType.Cone)
		{
			Center = center;
			Direction = direction;
			Length = length;
			PointsAffected = pointsAffected;
		}
	}

	public class SpecificAreaExplosionAnimationInfo : AnimationInfo
	{
		public ImmutableHashSet<Point> PointsAffected { get; }

		public SpecificAreaExplosionAnimationInfo (ImmutableHashSet<Point> pointsAffected) : base (AnimationType.SpecificAreaExplosion)
		{
			PointsAffected = pointsAffected;
		}
	}

	public interface IAnimationRequest
	{
		void Request (GameState state, AnimationInfo info);
		void RequestNewGame ();
		void RequestPlayerDead (GameState state);
		void RequestNewRound (GameState state, int round);
	}

	public class AnimationEventArgs : EventArgs
	{
		public GameState State { get; }
		public AnimationInfo Info { get; }

		public AnimationEventArgs (GameState state, AnimationInfo info)
		{
			State = state;
			Info = info;
		}
	}

	public class NewRoundEventArgs : EventArgs
	{
		public GameState State { get; }
		public int Round { get; }

		public NewRoundEventArgs (GameState state, int round)
		{
			State = state;
			Round = round;
		}
	}
}
