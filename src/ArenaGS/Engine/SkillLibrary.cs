using System;
using System.Collections.Generic;

using ArenaGS.Model;
using ArenaGS.Platform;

namespace ArenaGS.Engine
{
	public interface ISkillLibrary
	{
		Skill CreateSkill (string name);
	}
	
	public class SkillLibrary : ISkillLibrary
	{
		Dictionary<string, Skill> Skills = new Dictionary<string, Skill> ();
		IGenerator Generator;

		public SkillLibrary ()
		{
			Generator = Dependencies.Get<IGenerator> ();

			AddToLibrary (Generator.CreateSkill ("Dash", Effect.Movement, SkillEffectInfo.None, TargettingInfo.Point (2), SkillResources.WithRechargingAmmo (2, 3)));
			AddToLibrary (Generator.CreateSkill ("Point Blank Shot", Effect.Damage, new DamageSkillEffectInfo (0, knockback: true, stun: true), TargettingInfo.Point (2), SkillResources.WithAmmo (1)));
			AddToLibrary (Generator.CreateSkill ("Charge", Effect.Damage, new DamageSkillEffectInfo (1, charge: true), TargettingInfo.Point (2), SkillResources.WithCooldown (3)));
			AddToLibrary (Generator.CreateSkill ("Move & Shoot", Effect.MoveAndDamageClosest, new MoveAndDamageSkillEffectInfo (3, 3), TargettingInfo.Point (1), SkillResources.WithAmmo (4)));
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
