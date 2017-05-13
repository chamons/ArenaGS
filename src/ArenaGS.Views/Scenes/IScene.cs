using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SkiaSharp;

namespace ArenaGS.Views.Scenes
{
	internal interface IScene
	{
		void HandlePaint (SKSurface Surface);
		void HandleMouseDown (SKPointI point);
		void HandleMouseUp (SKPointI point);
		void HandleKeyDown (string character);
	}
}
