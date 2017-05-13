using System;

using AppKit;
using Foundation;
using SkiaSharp;
using ArenaGS.Views;
using SkiaSharp.Views.Mac;
using CoreGraphics;

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


	public partial class ViewController : NSViewController, IGameView {
		public ViewController (IntPtr handle) : base (handle)
		{
		}

		GameEngine Engine;
		PaintEventArgs PaintArgs = new PaintEventArgs ();
		ClickEventArgs ClickArgs = new ClickEventArgs ();
		KeyEventArgs KeyArgs = new KeyEventArgs ();

		public event EventHandler<PaintEventArgs> OnPaint;
		public event EventHandler<ClickEventArgs> OnMouseDown;
		public event EventHandler<ClickEventArgs> OnMouseUp;
		public event EventHandler<KeyEventArgs> OnKeyDown;

		SKCanvasView Canvas;
		public override void ViewDidLoad ()
		{
			base.ViewDidLoad ();

			Engine = new GameEngine (this);

			Canvas = new CanvasView (View.Frame);
			Canvas.PaintSurface += OnPlatformPaint;

			View.AddSubview (Canvas);

			Canvas.AutoresizingMask = NSViewResizingMask.MinXMargin | NSViewResizingMask.MinYMargin | 
				NSViewResizingMask.MaxXMargin | NSViewResizingMask.MaxYMargin | NSViewResizingMask.HeightSizable |
				NSViewResizingMask.WidthSizable;
		}

		public override void KeyDown (NSEvent theEvent)
		{
			base.KeyDown (theEvent);
			KeyArgs.Character = theEvent.Characters;
			OnKeyDown?.Invoke (this, KeyArgs);
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

		void OnPlatformPaint (object sender, SKPaintSurfaceEventArgs e)
		{
			PaintArgs.Surface = e.Surface;
			OnPaint?.Invoke (this, PaintArgs);
		}
	}
}
