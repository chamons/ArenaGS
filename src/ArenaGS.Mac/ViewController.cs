using System;

using AppKit;
using Foundation;
using SkiaSharp;
using ArenaGS.Views;
using SkiaSharp.Views.Mac;
using CoreGraphics;

namespace ArenaGS.Mac {
	public partial class ViewController : NSViewController, IGameView {
		public ViewController (IntPtr handle) : base (handle)
		{
		}

		GameEngine Engine;
		PaintEventArgs PaintArgs = new PaintEventArgs ();
		ClickEventArgs ClickArgs = new ClickEventArgs ();

		public event EventHandler<PaintEventArgs> OnPaint;
		public event EventHandler<ClickEventArgs> OnMouseDown;
		public event EventHandler<ClickEventArgs> OnMouseUp;

		SKCanvasView Canvas;
		public override void ViewDidLoad ()
		{
			base.ViewDidLoad ();

			Engine = new GameEngine (this);

			Canvas = new SKCanvasView (View.Frame);
			Canvas.PaintSurface += OnPlatformPaint;

			View.AddSubview (Canvas);

			Canvas.AutoresizingMask = NSViewResizingMask.MinXMargin | NSViewResizingMask.MinYMargin | 
				NSViewResizingMask.MaxXMargin | NSViewResizingMask.MaxYMargin | NSViewResizingMask.HeightSizable |
				NSViewResizingMask.WidthSizable;
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
