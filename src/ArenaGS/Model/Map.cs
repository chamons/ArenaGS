﻿using System.Text;
using ArenaGS.Utilities;

namespace ArenaGS.Model
{
	public class Map
	{
		public int Width { get; private set; }
		public int Height { get; private set; }

		MapTile[,] Tiles;

		public Map (int width, int height)
		{
			Width = width;
			Height = height;
			Tiles = new MapTile[width, height];
		}

		public MapTile this [int x, int y] => Tiles[x, y];
		public MapTile this [Point p] => Tiles[p.X, p.Y];

		public bool IsOnMap (Point p) => (p.X >= 0) && (p.Y >= 0) && (p.X < Width) && (p.Y < Height);
		public bool IsOnMap (int x, int y) => (x >= 0) && (y >= 0) && (x < Width) && (y < Height);

		internal void Set (Point p, TerrainType terrain)
		{
			Tiles [p.X, p.Y] = new MapTile (terrain);
		}

		public Point CoercePointOntoMap (Point p)
		{
			if (IsOnMap (p))
				return p;
			else
				return new Point (p.X.Clamp (0, Width - 1), p.Y.Clamp (0, Height - 1));
		}

		public override string ToString ()
		{
			StringBuilder output = new StringBuilder ();
			for (int j = 0; j < Height; ++j)
			{
				StringBuilder buffer = new StringBuilder (Width);
				for (int i = 0; i < Width; ++i)
					buffer.Append (Tiles[i, j].Terrain == TerrainType.Wall ? '#' : '.');
				output.AppendLine (buffer.ToString ());
			}
			output.AppendLine ();
			return output.ToString ();
		}
	}
}