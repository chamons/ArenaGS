using System.Collections.Generic;

using ArenaGS.Engine.Utilities;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;

using Optional;

namespace ArenaGS.Engine.Behavior
{
	public interface IActorBehavior
	{
		GameState Act (GameState state, Character c);
	}

	public class DefaultActorBehavior : IActorBehavior
	{		
		IPhysics Physics;

		public DefaultActorBehavior ()
		{
			Physics = Dependencies.Get<IPhysics> ();
		}

		public GameState Act (GameState state, Character c)
		{
			Option<GameState> walkState = WalkTowardsPlayerIfCan (state, c);

			return walkState.ValueOr (Physics.WaitEnemy (state, c));
		}

		Option<GameState> WalkTowardsPlayerIfCan (GameState state, Character c)
		{
			int[,] shortestPath = state.ShortestPath;

			List<Direction> nextStepsTowardPlayer = Dijkstra.NextStep (state.Map, shortestPath, c.Position);
			if (nextStepsTowardPlayer.Count == 0)
				return Option.None<GameState>();

			foreach (var direction in nextStepsTowardPlayer)
			{
				if (Physics.CouldCharacterWalk (state, c, c.Position.InDirection (direction)))
					return Physics.MoveEnemy (state, c, direction).Some ();
			}
			return Option.None<GameState> ();
		}
	}
}
