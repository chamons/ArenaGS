using System;
using ArenaGS.Model;
using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	internal class MapThemePainter
	{
		MapTileVariants TilesVariant = new MapTileVariants ();   

		SKBitmap GetFloor (MapTheme theme, Point currentModelPosition, Map map)
		{
			switch (theme)
			{
				case MapTheme.Beach:
					return Resources.Get ($"sand{TilesVariant.Get (map, currentModelPosition, 8, startAt: 1, rareAbove: 4)}.png");
				case MapTheme.Dungeon:
					return Resources.Get ($"pebble_brown{TilesVariant.Get (map, currentModelPosition, 8, rareAbove: 4)}.png");
				case MapTheme.FancyInside:
					return Resources.Get ($"crystal_floor{TilesVariant.Get (map, currentModelPosition, 6)}.png");
				case MapTheme.Mud:
					return Resources.Get ($"mud{TilesVariant.Get (map, currentModelPosition, 3)}.png");
				case MapTheme.Sandstone:
					return Resources.Get ($"sandstone_floor{TilesVariant.Get (map, currentModelPosition, 9, rareAbove: 5)}.png");
				default:
					throw new NotImplementedException ();
			}
		}

		SKBitmap GetWall (MapTheme theme, Point currentModelPosition, Map map)
		{
			switch (theme)
			{
				case MapTheme.Beach:
					return Resources.Get ($"stone2_brown{TilesVariant.Get (map, currentModelPosition, 4, rareAbove:0)}.png");
				case MapTheme.Dungeon:
					return Resources.Get ($"catacombs{TilesVariant.Get (map, currentModelPosition, 6, rareAbove:2)}.png");
				case MapTheme.FancyInside:
					return Resources.Get ($"stone2_gray{TilesVariant.Get (map, currentModelPosition, 4, rareAbove:0)}.png");
				case MapTheme.Mud:
					return Resources.Get ($"stone_gray{TilesVariant.Get (map, currentModelPosition, 4)}.png");
				case MapTheme.Sandstone:
					return Resources.Get ($"sandstone_wall{TilesVariant.Get (map, currentModelPosition, 5)}.png");
				default:
					throw new NotImplementedException ();
			}
		}

		SKBitmap GetDecoration (MapTheme theme, Point currentModelPosition, Map map)
		{
			switch (theme)
			{
				case MapTheme.Beach:
					return Resources.Get ($"mangrove{TilesVariant.Get (map, currentModelPosition, 3, startAt: 1)}.png");
				case MapTheme.Dungeon:
					return Resources.Get ($"crumbled_column_{TilesVariant.Get (map, currentModelPosition, 3, startAt: 1, rareAbove: 1)}.png");
				case MapTheme.FancyInside:
					return Resources.Get ($"crumbled_column_{TilesVariant.Get (map, currentModelPosition, 3, startAt: 1, rareAbove: 1)}.png");
				case MapTheme.Mud:
					return Resources.Get ($"mangrove{TilesVariant.Get (map, currentModelPosition, 3, startAt: 1)}.png");
				case MapTheme.Sandstone:
					return Resources.Get ($"crumbled_column_{TilesVariant.Get (map, currentModelPosition, 3, startAt: 1, rareAbove: 1)}.png");
				default:
					throw new NotImplementedException ();
			}
		}

		readonly string [] Statues = { "statue_angel.png", "statue_archer.png", "statue_archer.png", "statue_sword.png", "statue_twins.png" }; 
		readonly string [] Trees = { "tree1_lightred.png", "tree1_red.png", "tree1_yellow.png", "tree2_lightred.png", "tree2_red.png", "tree2_yellow.png" };
		readonly string [] OtherStatues = { "golden_statue_1.png", "golden_statue_2.png", "statue_iron_golem.png" };

		SKBitmap GetDecorationSpecial (MapTheme theme, Point currentModelPosition, Map map)
		{
			switch (theme)
			{
				case MapTheme.Beach:
					return Resources.Get ($"mangrove{TilesVariant.Get (map, currentModelPosition, 3, startAt: 1)}.png");
				case MapTheme.Dungeon:
					return Resources.Get (Statues [TilesVariant.Get (map, currentModelPosition, 5)]);
				case MapTheme.FancyInside:
					return Resources.Get (Statues [TilesVariant.Get (map, currentModelPosition, 5)]);
				case MapTheme.Mud:
					return Resources.Get (Trees [TilesVariant.Get (map, currentModelPosition, 6)]);
				case MapTheme.Sandstone:
					return Resources.Get (OtherStatues [TilesVariant.Get (map, currentModelPosition, 3)]);
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
