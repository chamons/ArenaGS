using System;
using System.Collections.Generic;

using ArenaGS.Model;
using ArenaGS.Platform;

namespace ArenaGS.Engine
{
	internal class SkillLibrary
	{
		Dictionary<string, Skill> Skills = new Dictionary<string, Skill> ();
		IGenerator Generator;

		public SkillLibrary (IGenerator generator)
		{
			Generator = generator;

			AddToLibrary (Generator.CreateSkill ("Aimed Shot", Effect.Damage, new DamageSkillEffectInfo (3), TargettingInfo.Point (5), SkillResources.None));
			AddToLibrary (Generator.CreateSkill ("Point Blank Shot", Effect.Damage, new DamageSkillEffectInfo (1, knockback: true, stun: true), TargettingInfo.Point (2), SkillResources.WithAmmo (1)));
			AddToLibrary (Generator.CreateSkill ("Move & Shoot", Effect.MoveAndDamageClosest, new MoveAndDamageSkillEffectInfo (2, 3), TargettingInfo.Point (1), SkillResources.WithCooldown (2)));

			AddToLibrary (Generator.CreateSkill ("Attack", Effect.Damage, new DamageSkillEffectInfo (3), TargettingInfo.Point (1), SkillResources.None));
			AddToLibrary (Generator.CreateSkill ("Stunning Strike", Effect.Damage, new DamageSkillEffectInfo (1, stun: true), TargettingInfo.Point (1), SkillResources.None));
			AddToLibrary (Generator.CreateSkill ("Power Slam", Effect.DelayedDamage, new DelayedDamageSkillEffectInfo (4), TargettingInfo.Line (4), SkillResources.WithCooldown (2)));

			AddToLibrary (Generator.CreateSkill ("Dash", Effect.Movement, SkillEffectInfo.None, TargettingInfo.Point (3), SkillResources.WithCooldown (6)));
			AddToLibrary (Generator.CreateSkill ("Charge", Effect.Damage, new DamageSkillEffectInfo (2, charge: true), TargettingInfo.Point (2), SkillResources.WithCooldown (3)));
			
		}

		void AddToLibrary (Skill s)
		{
			Skills.Add (s.Name, s);
		}

		public Skill CreateSkill (string name)
		{
			Skill skill;
			if (Skills.TryGetValue (name, out skill))
				return skill;
			throw new ArgumentException ($"Unknown skill {name} in library");
		}
	}
}
