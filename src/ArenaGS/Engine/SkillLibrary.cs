﻿using ArenaGS.Model;
using ArenaGS.Platform;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ArenaGS.Engine
{
	public class SkillLibrary
	{
		Dictionary<string, Skill> Skills = new Dictionary<string, Skill> ();
		IGenerator Generator;

		public SkillLibrary (IGenerator generator)
		{
			Generator = generator;

			AddToLibrary (Generator.CreateSkill ("Dash", Effect.Movement, new TargettingInfo (TargettingStyle.Point, 2), SkillResources.WithRechargingAmmo (2, 3), -1));
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