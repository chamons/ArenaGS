using System;
using System.Windows;
using ArenaGS.Views;
using SkiaSharp;
using ArenaGS.Utilities;

using Point = System.Windows.Point;

namespace ArenaGS.Windows
{
	public partial class MainWindow : Window, IGameWindow
	{
		GameController Controller;
		PaintEventArgs PaintArgs = new PaintEventArgs ();
		ClickEventArgs ClickArgs = new ClickEventArgs ();
		KeyEventArgs KeyArgs = new KeyEventArgs ();

		public event EventHandler<PaintEventArgs> OnPaint;
		public event EventHandler<ClickEventArgs> OnPress;
		public new event EventHandler<KeyEventArgs> OnKeyDown;
		public event EventHandler<ClickEventArgs> OnDetailPress;

		public event EventHandler<EventArgs> OnQuit;

		public MainWindow ()
		{
			InitializeComponent ();
			Loaded += OnLoaded;
			TextInput += OnPlatformTextEnter;
			KeyDown += OnPlatformKeyDown;
			Closed += OnPlatformClose;
		}

		public void Invalidate ()
		{
			SkiaView.InvalidateVisual ();
		}
			
		void OnLoaded (object sender, RoutedEventArgs e)
		{
			Controller = new GameController (this);
			Controller.Startup (new FileStorage ());
			SkiaView.InvalidateVisual ();
		}

		void OnPlatformPaint (object sender, SkiaSharp.Views.Desktop.SKPaintSurfaceEventArgs e)
		{
			PaintArgs.Surface = e.Surface;
			OnPaint?.Invoke (this, PaintArgs);
		}

		void OnPlatformMouseDown (object sender, System.Windows.Input.MouseButtonEventArgs e)
		{
			if (e.ChangedButton == System.Windows.Input.MouseButton.Right)
			{
				Point p = e.GetPosition (null);
				ClickArgs.Position = new SKPointI ((int)p.X, (int)p.Y);
				OnDetailPress?.Invoke (this, ClickArgs);
			}
		}

		void OnPlatformMouseUp (object sender, System.Windows.Input.MouseButtonEventArgs e)
		{
			if (e.ChangedButton == System.Windows.Input.MouseButton.Left)
			{
				Point p = e.GetPosition (null);
				ClickArgs.Position = new SKPointI ((int)p.X, (int)p.Y);
				OnPress?.Invoke (this, ClickArgs);
			}
		}

		void OnPlatformKeyDown (object sender, System.Windows.Input.KeyEventArgs e)
		{
			string entry = e.Key.ToString ();
			if (entry.Length > 1)
			{
				Console.WriteLine (entry);
				KeyArgs.Character = e.Key.ToString ();
				OnKeyDown?.Invoke (this, KeyArgs);
			}
		}

		void OnPlatformTextEnter (object sender, System.Windows.Input.TextCompositionEventArgs e)
		{
			string entry = e.TextComposition.Text;
			if (entry.Length == 1 && char.IsLetter (entry[0]))
			{
				Console.WriteLine (entry);
				KeyArgs.Character = entry;
				OnKeyDown?.Invoke (this, KeyArgs);
			}
		}

		void OnPlatformClose (object sender, EventArgs e)
		{
			OnQuit?.Invoke (this, e);
		}
	}
}
