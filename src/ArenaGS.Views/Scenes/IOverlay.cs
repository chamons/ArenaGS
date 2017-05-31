using ArenaGS.Views.Views;
using SkiaSharp;

namespace ArenaGS.Views.Scenes
{
	internal interface IOverlay
	{
		void HandleMouseDown (SKPointI point);
		void HandleMouseUp (SKPointI point);
		void HandleKeyDown (string character);

		void ConfigureView (CombatView combatView);
		void DisableOverlay (CombatView combatView);
	}
}
