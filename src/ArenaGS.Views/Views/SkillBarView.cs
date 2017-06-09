﻿using ArenaGS.Model;
using ArenaGS.Utilities;
using SkiaSharp;
using System;

namespace ArenaGS.Views.Views
{
	class SkillBarView : View
	{
		public SkillBarView (Point position, Size size) : base (position, size)
		{
		}

		const int MaxNumberOfSkills = 15;
		const int Padding = 2;
		const int CellSize = 32;
		const bool ShowHotkey = true;

		string [] CellLabels = new string [MaxNumberOfSkills] { "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "[", "]", "\\" };

		string ImageForSkill (Skill s)
		{
			switch (s.Name)
			{
				case "Fireball":
					return "burning-meteor.png";
				case "Grenade":
					return "grenade.png";
				default:
					return "cog.png";
			}			
		}

		SKPaint BlackPaint = new SKPaint () { Color = SKColors.Black };
		SKPaint AntialiasPaint = new SKPaint () { IsAntialias = false };
		SKPaint CellText = new SKPaint () { Color = SKColors.White, TextSize = 10, TextAlign = SKTextAlign.Center };
		SKPaint CellBorder = new SKPaint () { Color = SKColors.White, StrokeWidth = 2, IsStroke = true };

		public override SKSurface Draw (GameState state)
		{
			var skills = state.Player.Skills;
			for (int i = 0; i < Math.Min (skills.Count, MaxNumberOfSkills); ++i)
			{
				int left = Padding + ((Padding + CellSize) * i);
				int top = Padding;
				int right = left + CellSize + Padding;
				int bottom = top + CellSize + Padding;

				Canvas.DrawRect (new SKRect (left, top, right, bottom), CellBorder);

				Canvas.DrawBitmap (Resources.Get (ImageForSkill (skills[i])), new SKRect (left + Padding, top + Padding, right - Padding, bottom - Padding), AntialiasPaint);

				if (ShowHotkey)
				{
					int textLeft = left + (CellSize / 2);
					int textTop = CellSize + Padding + 3;
					Canvas.DrawRect (new SKRect (textLeft - 3, textTop - 8, textLeft + 4, textTop + 8), BlackPaint);
					Canvas.DrawText (CellLabels [i], textLeft, CellSize + Padding + 3, CellText);
				}
			}

			return Surface;
		}
	}
}
