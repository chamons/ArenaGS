using ArenaGS.Model;
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
				case "Aimed Shot":
					return "lee-enfield.png";
				case "Dash":
					return "run.png";
				case "Point Blank Shot":
					return "blunderbuss.png";
				case "Move & Shoot":
					return "crossed-pistols.png";
				default:
					return "cog.png";
			}			
		}

		SKPaint CooldownButUsablePaint = new SKPaint () { Color = SKColors.Black.WithAlpha (110) };
		SKPaint CooldownPaint = new SKPaint () { Color = SKColors.Black.WithAlpha (220) };
		SKPaint BlackPaint = new SKPaint () { Color = SKColors.Black.WithAlpha (220) };
		SKPaint AntialiasPaint = new SKPaint () { IsAntialias = false };
		SKPaint CellText = new SKPaint () { Color = SKColors.White, TextSize = 10, TextAlign = SKTextAlign.Center };
		SKPaint CellTextDark = new SKPaint () { Color = SKColors.White.WithAlpha (50), TextSize = 10, TextAlign = SKTextAlign.Center };
		SKPaint CellBorder = new SKPaint () { Color = SKColors.White, StrokeWidth = 2, IsStroke = true };

		SKRect RectForSkill (int i)
		{
			int left = Padding + ((Padding + CellSize) * i);
			int top = Padding;
			int right = left + CellSize + Padding;
			int bottom = top + CellSize + Padding;

			return new SKRect (left, top, right, bottom);
		}

		public override SKSurface Draw (GameState state)
		{
			base.Draw (state);

			var skills = state.Player.Skills;
			for (int i = 0; i < Math.Min (skills.Count, MaxNumberOfSkills); ++i)
			{
				Skill skill = skills [i];
				SKRect rect = RectForSkill (i);

				Canvas.DrawRect (rect, CellBorder);

				SKRect bitmapRect = new SKRect (rect.Left + Padding, rect.Top + Padding, rect.Right - Padding, rect.Bottom - Padding);
				Canvas.DrawBitmap (Resources.Get (ImageForSkill (skill)), bitmapRect, AntialiasPaint);

				bool skillDisabled = false;
				if (skill.UnderCooldown)
				{
					skillDisabled = true;
					float percentageLeft = (float)skill.Resources.Cooldown / (float)skill.Resources.MaxCooldown;
					float newHeight = bitmapRect.Height * percentageLeft;
					float remainingHeight = bitmapRect.Height - newHeight;
					SKPoint location = bitmapRect.Location;
					location.Offset (0, remainingHeight);
					SKRect cooldownRect = SKRect.Create (location, new SKSize (bitmapRect.Width, bitmapRect.Height * percentageLeft));
					Canvas.DrawRect (cooldownRect, skill.ReadyForUse ? CooldownButUsablePaint : CooldownPaint);
				}
				if (skill.UsesAmmo)
				{
					if (skill.Resources.CurrentAmmo == 0)
					{
						skillDisabled = true;
						Canvas.DrawRect (bitmapRect, CooldownPaint);
					}
					else
					{
						float textLeft = rect.Left + CellSize - 4;
						float textTop = CellSize + Padding - 1;
						Canvas.DrawRect (new SKRect (textLeft - 3, textTop - 9, textLeft + 5, textTop + 2), BlackPaint);
						Canvas.DrawText ($"{skill.Resources.CurrentAmmo}" , textLeft, textTop, CellText);
					}
				}

				if (ShowHotkey)
				{
					float textLeft = rect.Left + (CellSize / 2);
					float textTop = CellSize + Padding + 3;
					Canvas.DrawRect (new SKRect (textLeft - 3, textTop - 9, textLeft + 5, textTop + 2), BlackPaint);
					Canvas.DrawText (CellLabels [i], textLeft, textTop, skillDisabled ? CellTextDark : CellText);
				}
			}

			return Surface;
		}

		public override HitTestResults HitTest (SKPointI point)
		{
			if (!ScreenRect.Contains (point))
				return null;

			for (int i = 0 ; i < MaxNumberOfSkills ; ++i)
			{
				SKRect buttonRect = RectForSkill (i);
				buttonRect.Offset (Position.X, Position.Y);
				if (buttonRect.Contains (point))
					return new HitTestResults (this, i);
			}
			return null;
		}
	}
}
