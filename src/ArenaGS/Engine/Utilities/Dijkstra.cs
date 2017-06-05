﻿using System;
using System.Collections.Generic;
using System.Text;

using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Engine.Utilities
{
	public static class Dijkstra
	{
		struct MapNode
		{
			public Point Position;
			public int Value;

			public MapNode (Point p, int value)
			{
				Position = p;
				Value = value;
			}
		}

		static void AddNeighbors (MapNode current, Map map, Queue<MapNode> unvisited)
		{
			int newValue = current.Value + 1;
			foreach (Direction direction in Directions.All)
			{
				Point p = current.Position.InDirection (direction);
				if (map.IsWalkable (p))
					unvisited.Enqueue (new MapNode (p, newValue));
			}
		}

		public static int [,] CalculateShortestPathArray (Map map, Point initialPoint)
		{
			int [,] pathArray = new int [map.Width, map.Height];
			for (int i = 0; i < map.Width; ++i)
				for (int j = 0; j < map.Height; ++j)
					pathArray [i, j] = -1;

			var unvisited = new Queue<MapNode> ();
			unvisited.Enqueue (new MapNode (initialPoint, 0));

			while (unvisited.Count > 0)
			{
				var current = unvisited.Dequeue ();

				int existingValue = pathArray [current.Position.X, current.Position.Y];
				if (existingValue == -1)
				{
					pathArray [current.Position.X, current.Position.Y] = current.Value;
					AddNeighbors (current, map, unvisited);
				}
				else
				{
					pathArray [current.Position.X, current.Position.Y] = Math.Min (existingValue, current.Value);
				}
			}
			return pathArray;
		}

		public static List<Direction> NextStep (Map map, int [,] shortestPath, Point currentPoint)
		{
			int lowest = int.MaxValue;
			foreach (Direction direction in Directions.All)
			{
				Point adjPoint = currentPoint.InDirection (direction);
				if (map.IsOnMap (adjPoint))
				{
					int value = shortestPath [adjPoint.X, adjPoint.Y];
					if (value != -1)
						lowest = Math.Min (lowest, value);
				}
			}

			List<Direction> nextSteps = new List<Direction> ();
			foreach (Direction direction in Directions.All)
			{
				Point adjPoint = currentPoint.InDirection (direction);
				if (map.IsOnMap (adjPoint) && shortestPath [adjPoint.X, adjPoint.Y] == lowest)
					nextSteps.Add (direction);
			}
			return nextSteps;
		}

		public static string ToDebugString (this int [,] array)
		{
			int width = array.GetLength (0);
			int height = array.GetLength (1);

			StringBuilder output = new StringBuilder ();
			for (int j = 0; j < height; ++j)
			{
				StringBuilder buffer = new StringBuilder (width);
				for (int i = 0; i < width; ++i)
				{
					int value = array [i, j];
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
