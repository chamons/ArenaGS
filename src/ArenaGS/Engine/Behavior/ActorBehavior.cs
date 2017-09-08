using System.Collections.Generic;
using System.Linq;

using ArenaGS.Engine.Utilities;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;

using Optional;
using Optional.Unsafe;

namespace ArenaGS.Engine.Behavior
{
	public interface IActorBehavior
	{
		GameState Act (GameState state, Character c);
	}

	public class DefaultActorBehavior : IActorBehavior
	{		
		IPhysics Physics;
		ISkills Skills;

		public DefaultActorBehavior ()
		{
			Physics = Dependencies.Get<IPhysics> ();
			Skills = Dependencies.Get<ISkills> ();
		}

		public GameState Act (GameState state, Character c)
		{
			Option<GameState> healState = UseHealIfCan (state, c);
			if (healState.HasValue)
				return healState.ValueOrFailure ();
			
			Option<GameState> attackState = UseAttackIfCan (state, c);
			if (attackState.HasValue)
				return attackState.ValueOrFailure ();

			Option<GameState> movementState = UseMovementSkillIfCan (state, c);
			if (movementState.HasValue)
				return movementState.ValueOrFailure ();

			Option<GameState> walkState = WalkTowardsPlayerIfCan (state, c);
			if (walkState.HasValue)
				return walkState.ValueOrFailure ();

			return walkState.ValueOr (Physics.WaitEnemy (state, c));
		}


		Option<GameState> UseHealIfCan (GameState state, Character c)
		{
			return Option.None<GameState> ();
		}

		Option<GameState> UseAttackIfCan (GameState state, Character c)
		{
			return Option.None<GameState> ();
		}

		Option<GameState> UseMovementSkillIfCan (GameState state, Character c)
		{
			int [,] shortestPath = state.ShortestPath;
			int currentDistanceToPlayer = shortestPath [c.Position.X, c.Position.Y];

			int bestDistance = currentDistanceToPlayer;
			Option<Skill> bestSkill = Option.None<Skill> ();
			Point bestPoint = Point.Invalid;

			foreach (var skill in c.Skills.Where (x => x.Effect == Effect.Movement && x.ReadyForUse))
			{
				foreach (var targetablePoint in Skills.PointsSkillCanTarget (state, c, skill))
				{
					int currentSkillDistance = shortestPath [targetablePoint.X, targetablePoint.Y];
					if (currentSkillDistance < bestDistance)
					{
						bestSkill = skill.Some ();
						bestDistance = currentSkillDistance;
						bestPoint = targetablePoint;
					}
				}
			}

			if (bestSkill.HasValue)
				return Skills.Invoke (state, c, bestSkill.ValueOrFailure (), bestPoint).Some ();
			
			return Option.None<GameState> ();
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
