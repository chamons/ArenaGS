using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Utilities;
using System;
using System.Collections.Generic;

namespace ArenaGS.Tests.Utilities
{
	class TestCombat : ICombat
	{
		public List<Tuple<Character, int>> CharactersDamaged = new List<Tuple<Character, int>> ();
		public GameState Damage (GameState state, Character target, int amount)
		{
			CharactersDamaged.Add (new Tuple<Character, int> (target, amount));
			return state;
		}
	}
}
