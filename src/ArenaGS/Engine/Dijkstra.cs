using System;
using System.Collections.Generic;
using System.Text;
using System.Linq;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Engine
{
	public static class Dijkstra
	{
		static void AddNeighbors (Point current, Map map, Queue<KeyValuePair<Point, int>> unvisited, ref int[,] pathArray)
		{
			int newValue = pathArray[current.X, current.Y] + 1;
			foreach (Direction direction in Directions.All)
			{
				Point p = current.InDirection (direction);
				if (map.IsWalkable (p) && pathArray[p.X, p.Y] == -1 && !unvisited.Any (x => x.Key == p))
					unvisited.Enqueue (new KeyValuePair<Point, int> (p, newValue));
			}
		}
		
		public static int [,] CalculateShortestPathArray (Map map, Point initialPoint)
		{
			var unvisited = new Queue<KeyValuePair<Point, int>> ();

			int[,] pathArray = new int[map.Width, map.Height];
			for (int i = 0; i < map.Width; ++i)
				for (int j = 0; j < map.Height; ++j)
					pathArray[i, j] = -1;

			pathArray[initialPoint.X, initialPoint.Y] = 0;
			AddNeighbors (initialPoint, map, unvisited, ref pathArray);

			while (unvisited.Count > 0)
			{
				var current = unvisited.Dequeue ();
				pathArray[current.Key.X, current.Key.Y] = current.Value;
				AddNeighbors (current.Key, map, unvisited, ref pathArray);
			}

			return pathArray;
		}

		public static string ToDebugString (this int [,] array)
		{
			int width = array.GetLength (0);
			int height = array.GetLength (1);

			StringBuilder output = new StringBuilder ();
			for (int j = 0; j < height; ++j) {
				StringBuilder buffer = new StringBuilder (width);
				for (int i = 0; i < width; ++i) {
					int value = array[i, j];
					string symbol;
					if (value == -1)
						symbol = "*";
					else if (value < 10)
						symbol = value.ToString ();
					else
						symbol = ((char)((int)'A' + (value - 10))).ToString ();
					output.Append (symbol);
				}
				output.AppendLine (buffer.ToString ());
			}
			output.AppendLine ();
			return output.ToString ();
		}
	}
}
