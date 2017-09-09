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

			Option<GameState> movementAttack = UseMovementAttackIfCan (state, c);
			if (movementAttack.HasValue)
				return movementAttack.ValueOrFailure ();

			Option<GameState> attackState = UseAttackIfCan (state, c);
			if (attackState.HasValue)
				return attackState.ValueOrFailure ();

			Option<GameState> delayedAttackState = UseDelayAttackIfCan (state, c);
			if (delayedAttackState.HasValue)
				return delayedAttackState.ValueOrFailure ();

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

		IEnumerable<Skill> OrderHigestPower (IEnumerable<Skill> skills) => skills.OrderBy (x => x.EffectInfo.Power);
		Skill GetHigestPower (IEnumerable<Skill> skills) => OrderHigestPower (skills).LastOrDefault ();

		bool IsMeleeEnemy (Character c)
		{
			Skill strongestSkill = GetHigestPower (c.Skills);
			if (strongestSkill != null)
				return strongestSkill.TargetInfo.Range == 1;
			return true;
		}

		bool IsRangedEnemy (Character c) => !IsMeleeEnemy (c);

		Option<GameState> UseMovementAttackIfCan (GameState state, Character c)
		{
			int [,] shortestPath = state.ShortestPath;
			bool isMelee = IsMeleeEnemy (c);

			foreach (Skill skill in OrderHigestPower (c.Skills.Where (x => x.Effect == Effect.MoveAndDamageClosest && x.ReadyForUse)))
			{
				Point bestTarget = Point.Invalid;
				int bestDistance = shortestPath [c.Position.X, c.Position.Y];

				foreach (Point p in Skills.PointsSkillCanTarget (state, c, skill))
				{
					if (Skills.CharactersAffectedByMoveAndDamage (state, c, skill, p).Any (x => x.IsPlayer))
					{
						int currentDistance = shortestPath [p.X, p.Y];
						if ((isMelee && currentDistance < bestDistance) || (!isMelee && currentDistance > bestDistance))
						{
							bestTarget = p;
							bestDistance = currentDistance;
						}
					}
				}

				if (bestTarget != Point.Invalid)
					return Skills.Invoke (state, c, skill, bestTarget).Some ();
			}

			return Option.None<GameState> ();
		}

		Option<GameState> UseAttackIfCan (GameState state, Character c)
		{			
			var damageSkillsAvailable = c.Skills.Where (x => x.Effect == Effect.Damage && x.ReadyForUse);
			var damageSkillsWhichCanTarget = damageSkillsAvailable.Where (x => Skills.IsValidTarget (state, c, x, state.Player.Position));

			var stunDamageSkill = GetHigestPower (damageSkillsWhichCanTarget.Where (x => ((DamageSkillEffectInfo)x.EffectInfo).Stun));
			if (stunDamageSkill != null)
				return Skills.Invoke (state, c, stunDamageSkill, state.Player.Position).Some ();

			var bestDamageSkill = GetHigestPower (damageSkillsWhichCanTarget);
			if (bestDamageSkill != null)
				return Skills.Invoke (state, c, bestDamageSkill, state.Player.Position).Some ();

			return Option.None<GameState> ();
		}

		Option<GameState> UseDelayAttackIfCan (GameState state, Character c)
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
