using System;
using System.Collections.Generic;
using ArenaGS.Utilities;

namespace ArenaGS.Utilities
{
	public enum Direction
	{
		None, North, Northeast, East, Southeast, South, Southwest, West, Northwest
	}

	public static class Directions
	{
		public static readonly Direction[] All = new Direction [] { Direction.North, Direction.Northeast, Direction.East, Direction.Southeast,
				Direction.South, Direction.Southwest, Direction.West, Direction.Northwest};
	}

	public static class PointDirectionExtensions
	{
		public static Point InDirection (this Point initial, Direction direction)
		{
			switch (direction)
			{
				case Direction.None:
					return new Point (initial.X, initial.Y);
				case Direction.North:
					return new Point (initial.X, initial.Y - 1);
				case Direction.South:
					return new Point (initial.X, initial.Y + 1);
				case Direction.West:
					return new Point (initial.X - 1, initial.Y);
				case Direction.East:
					return new Point (initial.X + 1, initial.Y);
				case Direction.Northeast:
					return new Point (initial.X + 1, initial.Y - 1);
				case Direction.Northwest:
					return new Point (initial.X - 1, initial.Y - 1);
				case Direction.Southeast:
					return new Point (initial.X + 1, initial.Y + 1);
				case Direction.Southwest:
					return new Point (initial.X - 1, initial.Y + 1);
				default:
					throw new NotImplementedException ();
			}
		}

		public static Direction DirectionTo (this Point initial, Point end)
		{
			int x = end.X - initial.X;
			int y = end.Y - initial.Y;
			if (x > 1)
				x = 1;
			if (x < -1)
				x = -1;
			if (y > 1)
				y = 1;
			if (y < -1)
				y = -1;
			return ConvertPositionDeltaToDirection (x, y);
		}

		static Direction ConvertPositionDeltaToDirection (int deltaX, int deltaY)
		{
			if (deltaX == 1)
			{
				if (deltaY == 1)
					return Direction.Southeast;
				else if (deltaY == -1)
					return Direction.Northeast;
				else
					return Direction.East;
			}
			else if (deltaX == -1)
			{
				if (deltaY == 1)
					return Direction.Southwest;
				else if (deltaY == -1)
					return Direction.Northwest;
				else
					return Direction.West;
			}
			else
			{
				if (deltaY == 1)
					return Direction.South;
				else if (deltaY == -1)
					return Direction.North;
				else
					return Direction.None;
			}
		}

		public static int LatticeDistance (this Point point1, Point point2)
		{
			return Math.Abs (point1.X - point2.X) + Math.Abs (point1.Y - point2.Y);
		}

		public static double NormalDistance (this Point point1, Point point2)
		{
			return Math.Sqrt (Math.Pow (point1.X - point2.X, 2) + Math.Pow (point1.Y - point2.Y, 2));
		}
	}
}
