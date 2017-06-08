using System;
using SkiaSharp;

namespace ArenaGS.Views.Scenes
{
	internal interface IScene
	{
		void HandlePaint (SKSurface Surface);
		void HandleMouseDown (SKPointI point);
		void HandleMouseUp (SKPointI point);
		void HandleKeyDown (string character);

		bool AnimationInProgress { get; }
		void HandleAnimation (AnimationInfo info);
		event EventHandler AnimationsComplete;

		void Invalidate ();
	}
}
