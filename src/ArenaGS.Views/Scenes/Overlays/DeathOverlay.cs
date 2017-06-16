using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ArenaGS.Views.Views;
using SkiaSharp;

namespace ArenaGS.Views.Scenes.Overlays
{
	class DeathOverlay : IOverlay
	{
		GameEngine Engine;
		CombatScene Scene;
		IGameWindow Window;

		public DeathOverlay (GameEngine engine, CombatScene scene, IGameWindow window)
		{
			Engine = engine;
			Scene = scene;
			Window = window;
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
			map.DrawDeathNotice ();
		}

		public void HandleKeyDown (string character)
		{
			switch (character)
			{
				case "q":
					Window.Close ();
					break;
				case "n":
					Scene.SetDefaultOverlay ();
					Engine.AcceptCommand (Command.NewGame, null);
					break;
			}
		}

		public void OnPress (SKPointI point)
		{
		}
	}
}
