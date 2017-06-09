﻿using ArenaGS.Views.Views;
using SkiaSharp;

namespace ArenaGS.Views.Scenes
{
	internal interface IOverlay
	{
		void HandleMouseDown (SKPointI point);
		void HandleMouseUp (SKPointI point);
		void HandleKeyDown (string character);

		void DisableOverlay (CombatView combatView);

		void ConfigureMap (MapView map);
		void Draw (MapView map);
	}
}
