using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Utilities;
using System;
using System.Collections.Generic;

namespace ArenaGS.Tests.Utilities
{
	class TestPhysics : IPhysics
	{
		public GameState MovePlayer (GameState state, Direction direction) => throw new NotImplementedException ();
		public GameState MoveEnemy (GameState state, Character enemy, Direction direction) => throw new NotImplementedException ();
		public GameState WaitEnemy (GameState state, Character enemy) => throw new NotImplementedException ();
		public bool CouldCharacterWalk (GameState state, Character actor, Point newPosition) => throw new NotImplementedException ();

		public GameState Wait (GameState state, Character c) => state;
		public GameState WaitPlayer (GameState state) => state;

		public List<Tuple<Character, int>> CharactersDamaged = new List<Tuple<Character, int>> ();
		public GameState Damage (GameState state, Character target, int amount)
		{
			CharactersDamaged.Add (new Tuple<Character, int> (target, amount));
			return state;
		}
	}
}
