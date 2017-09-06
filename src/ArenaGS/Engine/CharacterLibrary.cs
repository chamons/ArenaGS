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
				Generator.CreateSkill ("Shot"),
				Generator.CreateSkill ("Dash"),
				Generator.CreateSkill ("Point Blank Shot"),
				Generator.CreateSkill ("Move & Shoot"),
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

