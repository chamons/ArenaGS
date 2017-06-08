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
		Explosion
	}

	public abstract class AnimationInfo
	{
		protected AnimationInfo (AnimationType type)
		{
			Type = type;
		}
		
		public AnimationType Type;
	}

	public class MovementAnimationInfo : AnimationInfo
	{
		public Character Character;
		public Point NewPosition;

		public MovementAnimationInfo (Character character, Point newPosition) : base (AnimationType. Movement)
		{
			Character = character;
			NewPosition = newPosition;
		}
	}

	public class ProjectileAnimationInfo : AnimationInfo
	{
		public ImmutableList <Point> Path;

		public ProjectileAnimationInfo (AnimationType type, List<Point> path) : base (AnimationType.Projectile)
		{
			Path = path.ToImmutableList ();
		}
	}

	public class ExplosionAnimationInfo : AnimationInfo
	{
		public Point Center;
		public int Size;

		public ExplosionAnimationInfo (Point center, int size) : base (AnimationType.Explosion)
		{
			Center = center;
			Size = size;
		}
	}

	public interface IAnimationRequest
	{
		void Request (GameState state, AnimationInfo info);
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
}
