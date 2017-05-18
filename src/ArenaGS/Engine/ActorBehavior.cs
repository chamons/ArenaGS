using System;
using ArenaGS.Model;

namespace ArenaGS.Engine
{
	public interface IActorBehavior
	{
		GameState Act (GameState state, Character c);
	}

	public class DefaultActorBehavior : IActorBehavior
	{
		public GameState Act (GameState state, Character c)
		{
			return state.WithReplaceEnemy (Physics.Wait (c));
		}
	}
}
