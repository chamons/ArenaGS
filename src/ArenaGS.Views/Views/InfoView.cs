using ArenaGS.Model;
using ArenaGS.Utilities;
using SkiaSharp;

namespace ArenaGS.Views.Views
{
	class InfoView : View
	{
		public InfoView (Point position, Size size) : base (position, size)
		{
		}

		public object Target { get; set; }

		const int InfoBorder = 2;
		const int TextXOffset = 6;
		const int TextYOffset = 20;
		const int FirstTextYOffset = 20;
		SKPaint TextPaint = new SKPaint () { Color = SKColors.White, TextSize = 12, IsAntialias = true, TextAlign = SKTextAlign.Left };

		void Write (string text, int offset = 1)
		{
			int verticalOffset = offset == 1 ? FirstTextYOffset : TextYOffset * offset;
			Canvas.DrawText (text, TextXOffset, verticalOffset, TextPaint);
		}

		public override SKSurface Draw (GameState state)
		{
			base.Draw (state);

			Canvas.DrawRect (new SKRect (0, 0, VisualRect.Width - InfoBorder, VisualRect.Height - InfoBorder), new SKPaint () { Color = SKColors.White, IsStroke = true });

			if (Target != null)
			{
				if (Target is Character character)
				{
					if (character.IsPlayer)
						Write ("Player");
					else
						Write ("Enemy");
					Write ($"Position: {character.Position}", 2);
					Write ($"Health: {character.Health.Current}/{character.Health.Maximum}", 3);
					Write ($"Defense: {character.Defense.StandardDefense}", 4);
				}
				else if (Target is Skill skill)
				{
					int currentOffset = 1;
					Write ($"Name: {skill.Name}", currentOffset);
					currentOffset += 1;

					switch (skill.TargetInfo.TargettingStyle)
					{
						case TargettingStyle.None:
							Write ($"Targetting: None.", currentOffset);
							break;
						case TargettingStyle.Point:
							if (skill.TargetInfo.Area > 1)
							{
								Write ($"Targetting:", currentOffset);
								currentOffset += 1;
								Write ($"     Burst of size {skill.TargetInfo.Area} to distance {skill.TargetInfo.Range}.", currentOffset);
							}
							else
							{
								Write ($"Targetting: Point up to distance {skill.TargetInfo.Range}.", currentOffset);
							}
							break;
						case TargettingStyle.Cone:
							Write ($"Targetting: Cone of distance {skill.TargetInfo.Range}.", currentOffset);
							break;
						case TargettingStyle.Line:
							Write ($"Targetting: Line of distance {skill.TargetInfo.Range}.", currentOffset);
							break;
					}
					currentOffset++;

					switch (skill.Effect)
					{
						case Effect.Damage:
						{
							DamageSkillEffectInfo skillInfo = (DamageSkillEffectInfo)skill.EffectInfo;
							Write ($"{skillInfo.Power} dice of damage.", currentOffset);
							currentOffset++;
							break;
						}
						case Effect.DelayedDamage:
						{
							DelayedDamageSkillEffectInfo skillInfo = (DelayedDamageSkillEffectInfo)skill.EffectInfo;
							Write ($"{skillInfo.Power} dice of damage after a delay.", currentOffset);
							currentOffset++;
							break;
						}
					}				

					if (skill.UsesAmmo)
					{
						Write ($"Charges {skill.Resources.CurrentAmmo}/{skill.Resources.MaxAmmo}.", currentOffset);
						currentOffset += 1;
					}
					if (skill.UsesCooldown)
					{
						if (skill.UnderCooldown)
							Write ($"Cooldown remaining {skill.Resources.Cooldown}/{skill.Resources.MaxCooldown}.", currentOffset);
						else
							Write ($"Cooldown when used: {skill.Resources.MaxCooldown}.", currentOffset);
						currentOffset += 1;
					}
					if (skill.RechargedAmmoOnCooldown)
					{
						Write ($"Skill recharges charges over time.", currentOffset);
						currentOffset += 1;
					}
				}
			}
			return Surface;
		}

		public override HitTestResults HitTest (SKPointI point)
		{
			return null;
		}
	}
}
