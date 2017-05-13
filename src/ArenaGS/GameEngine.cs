using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ArenaGS.Views;

namespace ArenaGS
{
    public class GameEngine
    {
		IGameView GameView;

		public GameEngine (IGameView gameView)
		{
			GameView = gameView;
			GameView.OnPaint += OnPaint;
			GameView.OnMouseDown += OnMouseDown;
			GameView.OnMouseUp += OnMouseUp;
			GameView.OnKeyDown += OnKeyDown;
		}

		void OnKeyDown (object sender, KeyEventArgs e)
		{
			System.Diagnostics.Debug.WriteLine ($"Down: {e.Character}");
		}

		void OnMouseUp (object sender, ClickEventArgs e)
		{
			System.Diagnostics.Debug.WriteLine ($"Down: {e.Position}");
		}

		void OnMouseDown (object sender, ClickEventArgs e)
		{
			System.Diagnostics.Debug.WriteLine ($"Up: {e.Position}");
		}

		void OnPaint (object sender, PaintEventArgs e)
		{
			e.Surface.Canvas.Clear (new SkiaSharp.SKColor (0, 0, 0));
		}
	}
}
