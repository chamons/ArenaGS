using System;

namespace ArenaGS.Utilities
{
	public static class PointExtensions
	{
		public static Point InDirection (this Point self, Direction direction)
		{
			switch (direction)
			{
				case Direction.North:
					return new Point (self.X, self.Y - 1);
				case Direction.South:
					return new Point (self.X, self.Y + 1);
				case Direction.West:
					return new Point (self.X - 1, self.Y);
				case Direction.East:
					return new Point (self.X + 1, self.Y);
				case Direction.Northeast:
					return new Point (self.X + 1, self.Y - 1);
				case Direction.Northwest:
					return new Point (self.X - 1, self.Y - 1);
				case Direction.Southeast:
					return new Point (self.X + 1, self.Y + 1);
				case Direction.Southwest:
					return new Point (self.X - 1, self.Y + 1);
				default:
					throw new ArgumentException ($"Unknown direction: {direction}");
			}
		}
	}
}
