﻿using System;
using System.Windows;
using ArenaGS.Views;
using SkiaSharp;

namespace ArenaGS.Windows
{
	public partial class MainWindow : Window, IGameView
	{
		GameEngine Engine;
		PaintEventArgs PaintArgs = new PaintEventArgs ();
		ClickEventArgs ClickArgs = new ClickEventArgs ();

		public event EventHandler<PaintEventArgs> OnPaint;
		public new event EventHandler<ClickEventArgs> OnMouseDown;
		public new event EventHandler<ClickEventArgs> OnMouseUp;

		public MainWindow ()
		{
			InitializeComponent ();
			Loaded += OnLoaded;
		}
			
		void OnLoaded (object sender, RoutedEventArgs e)
		{
			Engine = new GameEngine (this);
			SkiaView.InvalidateVisual ();
		}

		void OnPlatformPaint (object sender, SkiaSharp.Views.Desktop.SKPaintSurfaceEventArgs e)
		{
			PaintArgs.Surface = e.Surface;
			OnPaint?.Invoke (this, PaintArgs);
		}

		void OnPlatformMouseDown (object sender, System.Windows.Input.MouseButtonEventArgs e)
		{
			Point p = e.GetPosition (null);
			ClickArgs.Position = new SKPointI ((int)p.X, (int)p.Y);
			OnMouseDown?.Invoke (this, ClickArgs);
		}

		void OnPlatformMouseUp (object sender, System.Windows.Input.MouseButtonEventArgs e)
		{
			Point p = e.GetPosition (null);
			ClickArgs.Position = new SKPointI ((int)p.X, (int)p.Y);
			OnMouseUp?.Invoke (this, ClickArgs);
		}
	}
}
