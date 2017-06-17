using System;
using ArenaGS.Model;
using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	internal class MapThemeTiles
	{
		public string [] Floor { get; private set; }
		public string [] Wall { get; private set; }
		public string [] Decoration { get; private set; }
		public string [] DecorationSpecial { get; }

		private void CommonSetup (string floor, int floorMin, int floorCount, string wall, int wallMin, int wallCount, string decoration, int decorationMin, int decorationCount)
		{
			Floor = new string [floorCount + floorMin];
			for (int i = 0; i < floorCount + floorMin; ++i)
				Floor [i] = floor + i.ToString () + ".png";

			Wall = new string [wallCount + wallMin];
			for (int i = 0; i < wallCount + wallMin; ++i)
				Wall [i] = wall + i.ToString () + ".png";

			Decoration = new string [decorationCount + decorationMin];
			for (int i = 0; i < decorationCount + decorationMin; ++i)
				Decoration [i] = decoration + i.ToString () + ".png";
		}

		public MapThemeTiles (string floor, int floorMin, int floorCount, string wall, int wallMin, int wallCount, string decoration, int decorationMin, int decorationCount, string decorationSpecial, int decorationSpecialMin, int decorationSpecialCount)
		{
			CommonSetup (floor, floorMin, floorCount, wall, wallMin, wallCount, decoration, decorationMin, decorationCount);

			DecorationSpecial = new string [decorationSpecialCount + decorationSpecialMin];
			for (int i = 0; i < decorationSpecialCount + decorationSpecialMin; ++i)
				DecorationSpecial [i] = decorationSpecial + i.ToString () + ".png";
		}

		public MapThemeTiles (string floor, int floorMin, int floorCount, string wall, int wallMin, int wallCount, string decoration, int decorationMin, int decorationCount, string [] decorationSpecial)
		{
			CommonSetup (floor, floorMin, floorCount, wall, wallMin, wallCount, decoration, decorationMin, decorationCount);
			DecorationSpecial = decorationSpecial;
		}
	}

	internal class MapThemePainter
	{
		MapTileVariants TilesVariant = new MapTileVariants ();
		MapThemeTiles BeachTheme;
		MapThemeTiles DungeonTheme;
		MapThemeTiles FancyInsideTheme;
		MapThemeTiles MudTheme;
		MapThemeTiles SandstoneTheme;

		readonly string [] Statues = { "statue_angel.png", "statue_archer.png", "statue_archer.png", "statue_sword.png", "statue_twins.png" };
		readonly string [] Trees = { "tree1_lightred.png", "tree1_red.png", "tree1_yellow.png", "tree2_lightred.png", "tree2_red.png", "tree2_yellow.png" };
		readonly string [] OtherStatues = { "golden_statue_1.png", "golden_statue_2.png", "statue_iron_golem.png" };

		internal MapThemePainter ()
		{
			BeachTheme = new MapThemeTiles ("sand", 1, 8, "stone2_brown", 0, 4, "mangrove", 1, 3, "mangrove", 1, 3);
			DungeonTheme = new MapThemeTiles ("pebble_brown", 0, 8, "catacombs", 0, 6, "crumbled_column_", 1, 3, Statues);
			FancyInsideTheme = new MapThemeTiles ("crystal_floor", 0, 6, "stone2_gray", 0, 4, "crumbled_column_", 1, 3, Statues);
			MudTheme = new MapThemeTiles ("mud", 0, 3, "stone_gray", 0, 4, "mangrove", 1, 3, Trees);
			SandstoneTheme = new MapThemeTiles ("sandstone_floor", 0, 9, "sandstone_wall", 0, 5, "crumbled_column_", 1, 3, OtherStatues);
		}

		SKBitmap GetFloor (MapTheme theme, Point currentModelPosition, Map map)
		{
			switch (theme)
			{
				case MapTheme.Beach:
					return Resources.Get (BeachTheme.Floor [TilesVariant.Get (map, currentModelPosition, 8, startAt: 1, rareAbove: 4)]);
				case MapTheme.Dungeon:
					return Resources.Get (DungeonTheme.Floor [TilesVariant.Get (map, currentModelPosition, 8, rareAbove: 4)]);
				case MapTheme.FancyInside:
					return Resources.Get (FancyInsideTheme.Floor [TilesVariant.Get (map, currentModelPosition, 6)]);
				case MapTheme.Mud:
					return Resources.Get (MudTheme.Floor [TilesVariant.Get (map, currentModelPosition, 3)]);
				case MapTheme.Sandstone:
					return Resources.Get (SandstoneTheme.Floor [TilesVariant.Get (map, currentModelPosition, 9, rareAbove: 5)]);
				default:
					throw new NotImplementedException ();
			}
		}

		SKBitmap GetWall (MapTheme theme, Point currentModelPosition, Map map)
		{
			switch (theme)
			{
				case MapTheme.Beach:
					return Resources.Get (BeachTheme.Wall [TilesVariant.Get (map, currentModelPosition, 4, rareAbove: 0)]);
				case MapTheme.Dungeon:
					return Resources.Get (DungeonTheme.Wall [TilesVariant.Get (map, currentModelPosition, 6, rareAbove: 2)]);
				case MapTheme.FancyInside:
					return Resources.Get (FancyInsideTheme.Wall [TilesVariant.Get (map, currentModelPosition, 4, rareAbove: 0)]);
				case MapTheme.Mud:
					return Resources.Get (MudTheme.Wall [TilesVariant.Get (map, currentModelPosition, 4)]);
				case MapTheme.Sandstone:
					return Resources.Get (SandstoneTheme.Wall [TilesVariant.Get (map, currentModelPosition, 5)]);
				default:
					throw new NotImplementedException ();
			}
		}

		SKBitmap GetDecoration (MapTheme theme, Point currentModelPosition, Map map)
		{
			switch (theme)
			{
				case MapTheme.Beach:
					return Resources.Get (BeachTheme.Decoration [TilesVariant.Get (map, currentModelPosition, 3, startAt: 1)]);
				case MapTheme.Dungeon:
					return Resources.Get (DungeonTheme.Decoration [TilesVariant.Get (map, currentModelPosition, 3, startAt: 1, rareAbove: 1)]);
				case MapTheme.FancyInside:
					return Resources.Get (FancyInsideTheme.Decoration [TilesVariant.Get (map, currentModelPosition, 3, startAt: 1, rareAbove: 1)]);
				case MapTheme.Mud:
					return Resources.Get (MudTheme.Decoration [TilesVariant.Get (map, currentModelPosition, 3, startAt: 1)]);
				case MapTheme.Sandstone:
					return Resources.Get (SandstoneTheme.Decoration [TilesVariant.Get (map, currentModelPosition, 3, startAt: 1, rareAbove: 1)]);
				default:
					throw new NotImplementedException ();
			}
		}

		SKBitmap GetDecorationSpecial (MapTheme theme, Point currentModelPosition, Map map)
		{
			switch (theme)
			{
				case MapTheme.Beach:
					return Resources.Get (BeachTheme.DecorationSpecial [TilesVariant.Get (map, currentModelPosition, 3, startAt: 1)]);
				case MapTheme.Dungeon:
					return Resources.Get (DungeonTheme.DecorationSpecial [TilesVariant.Get (map, currentModelPosition, 5)]);
				case MapTheme.FancyInside:
					return Resources.Get (FancyInsideTheme.DecorationSpecial [TilesVariant.Get (map, currentModelPosition, 5)]);
				case MapTheme.Mud:
					return Resources.Get (MudTheme.DecorationSpecial [TilesVariant.Get (map, currentModelPosition, 6)]);
				case MapTheme.Sandstone:
					return Resources.Get (SandstoneTheme.DecorationSpecial [TilesVariant.Get (map, currentModelPosition, 3)]);
				default:
					throw new NotImplementedException ();
			}
		}

		// Not the fastest - https://github.com/chamons/ArenaGS/issues/51
		internal void DrawMapTile (MapView mapView, MapTheme theme, Point currentUIPosition, Point currentModelPosition, Map map)
		{
			var currentTile = map [currentModelPosition];
			switch (currentTile.Terrain)
			{
				case TerrainType.Floor:
					mapView.DrawTile (currentUIPosition, GetFloor (theme, currentModelPosition, map));
					return;
				case TerrainType.Wall:
					mapView.DrawTile (currentUIPosition, GetWall (theme, currentModelPosition, map));
					return;
				case TerrainType.Decoration:
					mapView.DrawTile (currentUIPosition, GetFloor (theme, currentModelPosition, map));
					mapView.DrawTile (currentUIPosition, GetDecoration (theme, currentModelPosition, map));
					return;
				case TerrainType.DecorationSpecial:
					mapView.DrawTile (currentUIPosition, GetFloor (theme, currentModelPosition, map));
					mapView.DrawTile (currentUIPosition, GetDecorationSpecial (theme, currentModelPosition, map));
					return;
				default:
					throw new NotImplementedException ();	
			}
		}
	}
}
