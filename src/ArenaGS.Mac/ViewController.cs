using System;

using AppKit;
using CoreGraphics;
using Foundation;

using ArenaGS.Utilities;
using ArenaGS.Views;
using SkiaSharp;
using SkiaSharp.Views.Mac;

namespace ArenaGS.Mac {
	public class CanvasView : SKCanvasView
	{
		public CanvasView (IntPtr p) : base (p)
		{
		}

		public CanvasView (CGRect r) : base (r)
		{
		}

		public override bool AcceptsFirstResponder ()
		{
			return true;
		}
	}


	public partial class ViewController : NSViewController, INSWindowDelegate, IGameWindow {
		public ViewController (IntPtr handle) : base (handle)
		{
		}

		GameController Controller;
		PaintEventArgs PaintArgs = new PaintEventArgs ();
		ClickEventArgs ClickArgs = new ClickEventArgs ();
		KeyEventArgs KeyArgs = new KeyEventArgs ();

		public event EventHandler<PaintEventArgs> OnPaint;
		public event EventHandler<ClickEventArgs> OnMouseDown;
		public event EventHandler<ClickEventArgs> OnMouseUp;
		public event EventHandler<KeyEventArgs> OnKeyDown;
		public event EventHandler<EventArgs> OnQuit;

		SKCanvasView Canvas;
		public override void ViewDidLoad ()
		{
			base.ViewDidLoad ();

			Controller = new GameController (this);
			Controller.Startup (new FileStorage ());

			Canvas = new CanvasView (View.Frame);
			Canvas.PaintSurface += OnPlatformPaint;

			View.AddSubview (Canvas);

			Canvas.AutoresizingMask = NSViewResizingMask.MinXMargin | NSViewResizingMask.MinYMargin | 
				NSViewResizingMask.MaxXMargin | NSViewResizingMask.MaxYMargin | NSViewResizingMask.HeightSizable |
				NSViewResizingMask.WidthSizable;
		}

		public override void ViewDidAppear ()
		{
			base.ViewDidAppear ();
			View.Window.Delegate = this;
		}

		[Export ("windowShouldClose:")]
		public bool WindowShouldClose (NSObject sender)
		{
			OnQuit?.Invoke (this, EventArgs.Empty);
			return true;
		}

		public override void KeyDown (NSEvent theEvent)
		{
			base.KeyDown (theEvent);
			KeyArgs.Character = ConvertNSEventToKeyString(theEvent);
			OnKeyDown?.Invoke (this, KeyArgs);
		}

		string ConvertNSEventToKeyString (NSEvent theEvent)
		{
			switch (theEvent.KeyCode)
			{
				case (ushort)NSKey.UpArrow:
				case (ushort)NSKey.Keypad8:
					return "Up";
				case (ushort)NSKey.DownArrow:
				case (ushort)NSKey.Keypad2:
					return "Down";
				case (ushort)NSKey.LeftArrow:
				case (ushort)NSKey.Keypad4:
					return "Left";
				case (ushort)NSKey.RightArrow:
				case (ushort)NSKey.Keypad6:
					return "Right";
				default:
					return theEvent.Characters;
			}
		}

		public override void MouseDown (NSEvent theEvent)
		{
			CGPoint p = theEvent.LocationInWindow;
			ClickArgs.Position = new SKPointI ((int)p.X, (int)p.Y);
			OnMouseDown?.Invoke (this, ClickArgs);	
		}

		public override void MouseUp (NSEvent theEvent)
		{
			CGPoint p = theEvent.LocationInWindow;
			ClickArgs.Position = new SKPointI ((int)p.X, (int)p.Y);
			OnMouseUp?.Invoke (this, ClickArgs);
		}

		public void Invalidate ()
		{
			if (Canvas != null)
				Canvas.NeedsDisplay = true;
		}

		void OnPlatformPaint (object sender, SKPaintSurfaceEventArgs e)
		{
			PaintArgs.Surface = e.Surface;
			OnPaint?.Invoke (this, PaintArgs);
		}
	}
}
