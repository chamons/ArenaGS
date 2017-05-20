using System;
using System.Collections.Generic;

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
	}
}
