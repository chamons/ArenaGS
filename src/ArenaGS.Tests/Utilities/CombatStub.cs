using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Utilities;
using System;
using System.Collections.Generic;

namespace ArenaGS.Tests.Utilities
{
	class CombatStub : ICombat
	{
		public List<Tuple<Character, int>> CharactersDamaged = new List<Tuple<Character, int>> ();
		public GameState Damage (GameState state, Character target, int amount)
		{
			CharactersDamaged.Add (new Tuple<Character, int> (target, amount));
			return state;
		}

		public List<Tuple<Character, int>> CharactersHealed = new List<Tuple<Character, int>> ();
		public GameState Heal (GameState state, Character target, int amount)
		{
			CharactersHealed.Add (new Tuple<Character, int> (target, amount));
			return state;
		}
	}
}
