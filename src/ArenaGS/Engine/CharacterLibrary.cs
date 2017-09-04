using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;

namespace ArenaGS
{
	public interface ICharacterLibrary
	{
		Character CreateCharacter (string name);
	}
	
	public class CharacterLibrary : ICharacterLibrary
	{
		Dictionary<string, Character> Characters = new Dictionary<string, Character> ();

		IGenerator Generator;

		public CharacterLibrary ()
		{
			Generator = Dependencies.Get <IGenerator> ();

			Character player = Generator.CreatePlayer (Point.Empty, new Health (3, 3), new Defense (1));
			player = player.WithSkills (new Skill[] {
				Generator.CreateSkill ("Shot", Effect.Damage, new DamageSkillEffectInfo (2), TargettingInfo.Point (8), new SkillResources (maxCooldown : 3)),
				Generator.CreateSkill ("Grenade", Effect.Damage, new DamageSkillEffectInfo (1), TargettingInfo.Point (4, 3), new SkillResources (maxAmmo : 2)),
				Generator.CreateSkill ("Dragon's Breath", Effect.Damage, new DamageSkillEffectInfo (1), TargettingInfo.Cone (3), new SkillResources (maxCooldown : 5)),
				Generator.CreateSkill ("Delayed Blast", Effect.DelayedDamage, new DelayedDamageSkillEffectInfo (3), TargettingInfo.Point (3, 1), new SkillResources (maxAmmo : 2)),
				Generator.CreateSkill ("Line Strike", Effect.Damage, new DamageSkillEffectInfo (3), TargettingInfo.Line (3), new SkillResources (maxCooldown : 2)),
				Generator.CreateSkill ("Dash"),
				Generator.CreateSkill ("Point Blank Shot"),
				Generator.CreateSkill ("Charge"),
				Generator.CreateSkill ("Move & Shoot")
			}.ToImmutableList ());
			AddToLibrary (player);
		}

		void AddToLibrary (Character e)
		{
			Characters.Add (e.Name, e);
		}

		public Character CreateCharacter (string name)
		{
			Character character;
			if (Characters.TryGetValue (name, out character))
				return character;
			throw new ArgumentException ($"Unknown character {name} in library");
		}
	}
}

