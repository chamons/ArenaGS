using System;
using ArenaGS.Views;
using ArenaGS.Views.Scenes;

namespace ArenaGS
{
	public class GameController
    {
		IGameWindow GameWindow;
		GameEngine GameEngine;
		IScene CurrentScene;

		public GameController (IGameWindow gameWindow)
		{
			GameWindow = gameWindow;
			GameWindow.OnPaint += OnPaint;
			GameWindow.OnMouseDown += OnMouseDown;
			GameWindow.OnMouseUp += OnMouseUp;
			GameWindow.OnKeyDown += OnKeyDown;
		}

		public void Startup ()
		{
			GameEngine = new GameEngine ();

			// TODO - This will someday need to be calculated based on current GameState
			CurrentScene = new CombatScene (GameEngine);

			GameEngine.StateChanged += OnGameEngineStateChanged;
			GameEngine.LoadGame ();
		}

		private void OnGameEngineStateChanged (object sender, EventArgs e)
		{
			// This is lazy and will need to be changed, specially when we have animations
			GameWindow.Invalidate ();
		}

		void OnKeyDown (object sender, KeyEventArgs e)
		{
			CurrentScene.HandleKeyDown (e.Character);
		}

		void OnMouseUp (object sender, ClickEventArgs e)
		{
			CurrentScene.HandleMouseUp (e.Position);
		}

		void OnMouseDown (object sender, ClickEventArgs e)
		{
			CurrentScene.HandleMouseDown (e.Position);
		}

		void OnPaint (object sender, PaintEventArgs e)
		{
			CurrentScene.HandlePaint (e.Surface);
		}
	}
}
