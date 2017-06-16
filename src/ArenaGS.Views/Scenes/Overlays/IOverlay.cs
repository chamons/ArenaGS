﻿using ArenaGS.Views.Views;
using SkiaSharp;

namespace ArenaGS.Views.Scenes.Overlays
{
	internal interface IOverlay
	{
		void OnPress (SKPointI point);
		void HandleKeyDown (string character);

		void BeforeDisabled (CombatView combatView);

		void ConfigureMapForDrawing (MapView map);
		void Draw (MapView map);

		object InfoTarget { get; }
	}
}
