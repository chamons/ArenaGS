using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SkiaSharp;

namespace ArenaGS.Views
{
	public class PaintEventArgs : EventArgs
	{
		public SKSurface Surface { get; set; }
	}

	public class ClickEventArgs : EventArgs
	{
		public SKPointI Position { get; set; }
	}

	public class KeyEventArgs : EventArgs
	{
		public char Character { get; set; }
	}

	public interface IGameView
    {
		event EventHandler<PaintEventArgs> OnPaint;
		event EventHandler<ClickEventArgs> OnMouseDown;
		event EventHandler<ClickEventArgs> OnMouseUp;
		event EventHandler<KeyEventArgs> OnKeyDown;
	}
}
