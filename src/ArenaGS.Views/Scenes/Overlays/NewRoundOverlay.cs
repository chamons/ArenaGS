using ArenaGS.Views.Views;
using SkiaSharp;

namespace ArenaGS.Views.Scenes.Overlays
{
	class NewRoundOverlay : IOverlay
	{
		GameEngine Engine;
		CombatScene Scene;
		IGameWindow Window;
		int Round;

		public NewRoundOverlay (GameEngine engine, CombatScene scene, IGameWindow window, int round)
		{
			Engine = engine;
			Scene = scene;
			Window = window;
			Round = round;
		}

		public object InfoTarget => null;

		public void ConfigureMapForDrawing (MapView map)
		{
		}

		public void BeforeDisabled (CombatView combatView)
		{
		}

		public void Draw (MapView map)
		{
			map.DrawNewRound (Round);
		}

		public void HandleKeyDown (string character)
		{
			Scene.SetDefaultOverlay ();
		}

		public void OnPress (SKPointI point)
		{
			Scene.SetDefaultOverlay ();
		}
	}
}
