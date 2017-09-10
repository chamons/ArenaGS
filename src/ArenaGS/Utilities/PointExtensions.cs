using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace ArenaGS.Utilities
{
	public static class PointExtensions
	{
		public static Direction InOppositeDirection (this Direction direction)
		{
			switch (direction)
			{
				case Direction.North:
					return Direction.South;
				case Direction.South:
					return Direction.North;
				case Direction.West:
					return Direction.East;
				case Direction.East:
					return Direction.West;
				case Direction.Northeast:
					return Direction.Southwest;
				case Direction.Northwest:
					return Direction.Northeast;
				case Direction.Southeast:
					return Direction.Northwest;
				case Direction.Southwest:
					return Direction.Northeast;
				default:
					throw new NotImplementedException ();
			}
		}

		public static IEnumerable<Direction> DirectionsNearby (this Direction direction)
		{
			switch (direction)
			{
				case Direction.North:
					return new Direction[] { Direction.Northwest, Direction.Northeast };
				case Direction.South:
					return new Direction[] { Direction.Southwest, Direction.Southeast };
				case Direction.West:
					return new Direction[] { Direction.Northwest, Direction.Southwest };
				case Direction.East:
					return new Direction[] { Direction.Northeast, Direction.Southeast };
				case Direction.Northeast:
					return new Direction[] { Direction.North, Direction.East };
				case Direction.Northwest:
					return new Direction[] { Direction.North, Direction.West };
				case Direction.Southeast:
					return new Direction[] { Direction.South, Direction.East };
				case Direction.Southwest:
					return new Direction[] { Direction.South, Direction.West };
				default:
					throw new NotImplementedException ();
			}
		}

		public static IEnumerable<Direction> DirectionsAway (this Direction direction)
		{
			switch (direction)
			{
				case Direction.North:
					return new Direction[] { Direction.Southwest, Direction.South, Direction.Southeast };
				case Direction.South:
					return new Direction[] { Direction.Northwest, Direction.North, Direction.Northeast };
				case Direction.West:
					return new Direction[] { Direction.Northeast, Direction.East, Direction.Southeast };
				case Direction.East:
					return new Direction[] { Direction.Northwest, Direction.West, Direction.Southwest };
				case Direction.Northeast:
					return new Direction[] { Direction.West, Direction.Southwest, Direction.South };
				case Direction.Northwest:
					return new Direction[] { Direction.East, Direction.Southeast, Direction.South };
				case Direction.Southeast:
					return new Direction[] { Direction.Northwest, Direction.North, Direction.West };
				case Direction.Southwest:
					return new Direction[] { Direction.Northeast, Direction.North, Direction.East };
				default:
					throw new NotImplementedException ();
			}
		}

		public static IEnumerable<Point> PointsInBurst (this Point center, int burstDistance)
		{
			List<Point> points = new List<Point> ();

			for (int i = -burstDistance; i <= burstDistance; ++i)
			{
				for (int j = -burstDistance; j <= burstDistance; ++j)
				{
					int distanceFromCenter = Math.Abs (i) + Math.Abs (j);
					if (distanceFromCenter <= burstDistance)
						points.Add (new Point (center.X + i, center.Y + j));
				}
			}
			return points;
		}

		public static IEnumerable<Point> PointsInGrid (this Point center, int radius)
		{
			List<Point> points = new List<Point> ();
			for (int i = -radius; i <= radius; ++i)
				for (int j = -radius; j <= radius; ++j)
					points.Add (new Point (center.X + i, center.Y + j));
			return points;
		}

		public static IEnumerable<Point> PointsInCone (this Point center, Direction direction, int coneLength)
		{
			if (direction == Direction.None)
				return Enumerable.Empty<Point> ();

			if (direction == Direction.Northeast || direction == Direction.Northwest || direction == Direction.Southeast || direction == Direction.Southwest)
				throw new NotImplementedException ();

			List<Point> affectedPoints = new List<Point> ();

			Point firstPointInDirection = center.InDirection (direction);
			if (center == firstPointInDirection)
				return affectedPoints;

			int deltaX = firstPointInDirection.X - center.X;
			int deltaY = firstPointInDirection.Y - center.Y;
			Point coneCenterForDistance = firstPointInDirection;
			for (int i = 0; i < coneLength; ++i)
			{
				affectedPoints.Add (coneCenterForDistance);
				for (int z = 0; z < i + 1; ++z)
				{
					if (deltaX != 0)
					{
						affectedPoints.Add (new Point (coneCenterForDistance.X, coneCenterForDistance.Y - (z + 1)));
						affectedPoints.Add (new Point (coneCenterForDistance.X, coneCenterForDistance.Y + (z + 1)));
					}
					else
					{
						affectedPoints.Add (new Point (coneCenterForDistance.X - (z + 1), coneCenterForDistance.Y));
						affectedPoints.Add (new Point (coneCenterForDistance.X + (z + 1), coneCenterForDistance.Y));
					}
				}
				coneCenterForDistance = new Point (coneCenterForDistance.X + deltaX, coneCenterForDistance.Y + deltaY);
			}
			return affectedPoints;
		}
	}
}
