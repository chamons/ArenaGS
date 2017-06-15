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
		public string Character { get; set; }
	}

	public interface IGameWindow
	{
		void Invalidate ();

		event EventHandler<PaintEventArgs> OnPaint;
		event EventHandler<ClickEventArgs> OnDetailPress;
		event EventHandler<ClickEventArgs> OnPress;
		event EventHandler<KeyEventArgs> OnKeyDown;
		event EventHandler<EventArgs> OnQuit;
	}
}
