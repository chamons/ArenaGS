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

			Canvas = new CanvasView (View.Frame) { IgnorePixelScaling = true };
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
			KeyArgs.Character = ConvertNSEventToKeyString(theEvent);
			OnKeyDown?.Invoke (this, KeyArgs);
		}

		string ConvertNSEventToKeyString (NSEvent theEvent)
		{
			switch (theEvent.KeyCode)
			{
				case (ushort)NSKey.UpArrow:
					return "Up";
				case (ushort)NSKey.DownArrow:
					return "Down";
				case (ushort)NSKey.LeftArrow:
					return "Left";
				case (ushort)NSKey.RightArrow:
					return "Right";
				case (ushort)NSKey.Keypad1:
					return "NumPad1";
				case (ushort)NSKey.Keypad2:
					return "NumPad2";
				case (ushort)NSKey.Keypad3:
					return "NumPad3";
				case (ushort)NSKey.Keypad4:
					return "NumPad4";
				case (ushort)NSKey.Keypad5:
					return "NumPad5";
				case (ushort)NSKey.Keypad6:
					return "NumPad6";
				case (ushort)NSKey.Keypad7:
					return "NumPad7";
				case (ushort)NSKey.Keypad8:
					return "NumPad8";
				case (ushort)NSKey.Keypad9:
					return "NumPad9";
				default:
					return theEvent.Characters;
			}
		}

		public override void MouseDown (NSEvent theEvent)
		{
			CGPoint p = theEvent.LocationInWindow;
			ClickArgs.Position = new SKPointI ((int)p.X, (int)View.Frame.Height - (int)p.Y);
			OnMouseDown?.Invoke (this, ClickArgs);	
		}

		public override void MouseUp (NSEvent theEvent)
		{
			CGPoint p = theEvent.LocationInWindow;
			ClickArgs.Position = new SKPointI ((int)p.X, (int)View.Frame.Height - (int)p.Y);
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
