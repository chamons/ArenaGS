﻿using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

using ArenaGS.Engine.Utilities;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;

namespace ArenaGS.Engine
{
	public interface ISkills
	{
		GameState Invoke (GameState state, Character invoker, Skill skill, Point target);
		bool IsValidTarget (GameState state, Character invoker, Skill skill, Point target);
		HashSet<Point> AffectedPointsForSkill (GameState state, Character invoker, Skill skill, Point target);
		HashSet<Point> PointsSkillCanTarget (GameState state, Character invoker, Skill skill);
		IEnumerable<Character> CharactersAffectedByMoveAndDamage (GameState state, Character invoker, Skill skill, Point position);
	}

	public class Skills : ISkills
	{
		IPhysics Physics;
		ICombat Combat;
		IAnimationRequest Animation;
		IGenerator Generator;
	
		public Skills ()
		{
			Physics = Dependencies.Get<IPhysics> ();
			Combat = Dependencies.Get<ICombat> ();
			Animation = Dependencies.Get<IAnimationRequest> ();
			Generator = Dependencies.Get<IGenerator> ();
		}

		public GameState Invoke (GameState state, Character invoker, Skill skill, Point target)
		{
			if (!invoker.Skills.Contains (skill))
				throw new InvalidOperationException ($"{invoker} tried to invoke {skill.Name} but did not contain it.");

			if (!IsValidTarget (state, invoker, skill, target))
				throw new InvalidOperationException ($"{invoker} tried to invoke {skill.Name} at {target} but was invalid.");

			if (!skill.ReadyForUse)
				throw new InvalidOperationException ($"{invoker} tried to invoke {skill.Name} but was not ready for use: {skill.Resources}.");

			switch (skill.Effect)
			{
				case Effect.Damage:
				{
					state = HandleDamageSkill (state, invoker, skill, target);
					break;
				}
				case Effect.DelayedDamage:
				{
					DelayedDamageSkillEffectInfo effectInfo = (DelayedDamageSkillEffectInfo)skill.EffectInfo;

					HashSet<Point> areaAffected = AffectedPointsForSkill (state, invoker, skill, target);
					state = state.WithAddedScript (Generator.CreateDamageScript (-100, effectInfo.Power, areaAffected.ToImmutableHashSet ()));
					break;
				}
				case Effect.Movement:
				{
					state = HandleMovement (state, invoker, target);
					break;
				}
				case Effect.MoveAndDamageClosest:
					{
						MoveAndDamageSkillEffectInfo effectInfo = (MoveAndDamageSkillEffectInfo)skill.EffectInfo;
						state = HandleMovement (state, invoker, target);
						invoker = state.UpdateCharacterReference (invoker);

						IEnumerable<Character> potentialTargets = CharactersAffectedByMoveAndDamage (state, invoker, skill, invoker.Position);
						var finalTarget = potentialTargets.FirstOrDefault ();

						if (finalTarget != null)
						{
							List<Point> path = BresenhamLine.PointsOnLine (invoker.Position, finalTarget.Position);
							if (path.Count > 0)
								Animation.Request (state, new ProjectileAnimationInfo (path));
							state = Combat.Damage (state, finalTarget, effectInfo.Power);
						}
						break;
					}
				case Effect.Heal:
				{
					HealEffectInfo effectInfo = (HealEffectInfo)skill.EffectInfo;
					HashSet<Point> areaAffected = AffectedPointsForSkill (state, invoker, skill, target);
					var charactersInRange = state.AllCharacters.Where (x => areaAffected.Contains (x.Position));
					var charactersAffected = charactersInRange.Where (x => x.IsPlayer == invoker.IsPlayer);  // #105
					foreach (var enemy in charactersAffected)
						state = Combat.Heal (state, enemy, effectInfo.Power);
					break;
				}
				case Effect.None:
					break;
			}

			invoker = state.UpdateCharacterReference (invoker);
			state = ChargeSkillForResources (state, invoker, skill);
			invoker = state.UpdateCharacterReference (invoker);

			return Physics.Wait (state, invoker).WithNewLogLine ($"Skill: {skill.Name} at {target}");
		}

		GameState HandleMovement (GameState state, Character invoker, Point target)
		{
			if (state.AllCharacters.Any (x => x.Position == target))
				throw new InvalidOperationException ($"{invoker} tried to invoke movement skill to {target} but was invalid as it was already occupied.");

			Animation.Request (state, new MovementAnimationInfo (invoker, target));
			return state.WithReplaceCharacter (invoker.WithPosition (target));
		}

		GameState HandleDamageSkill (GameState state, Character invoker, Skill skill, Point target)
		{
			HashSet<Point> areaAffected = AffectedPointsForSkill (state, invoker, skill, target);
			DamageSkillEffectInfo effectInfo = (DamageSkillEffectInfo)skill.EffectInfo;

			switch (skill.TargetInfo.TargettingStyle)
			{
				case TargettingStyle.Point:
				{
					List<Point> path = BresenhamLine.PointsOnLine (invoker.Position, target);
					if (!effectInfo.Charge && path.Count > 0)
						Animation.Request (state, new ProjectileAnimationInfo (path));

					if (skill.TargetInfo.Area > 1)
						Animation.Request (state, new ExplosionAnimationInfo (target, skill.TargetInfo.Area, areaAffected.ToImmutableHashSet ()));

					if (effectInfo.Charge && path.Count > 1)
					{
						Point locationNextToTarget = path [path.Count - 2];
						Animation.Request (state, new MovementAnimationInfo (invoker, locationNextToTarget));
						state = state.WithReplaceCharacter (invoker.WithPosition (locationNextToTarget));
						invoker = state.UpdateCharacterReference (invoker);
					}

					break;
				}
				case TargettingStyle.Cone:
				{
					Direction direction = invoker.Position.DirectionTo (target);
					Animation.Request (state, new ConeAnimationInfo (invoker.Position, direction, skill.TargetInfo.Range, areaAffected.ToImmutableHashSet ()));
					break;
				}
				case TargettingStyle.Line:
				{
					Animation.Request (state, new SpecificAreaExplosionAnimationInfo (areaAffected.ToImmutableHashSet ()));
					break;
				}
			}

			foreach (var enemy in state.AllCharacters.Where (x => areaAffected.Contains (x.Position)))
				state = Combat.Damage (state, enemy, effectInfo.Power);

			if (effectInfo.Stun)
			{
				foreach (var enemy in state.AllCharacters.Where (x => areaAffected.Contains (x.Position)).ToList ())
					state = state.WithReplaceCharacter (enemy.WithAdditionalCT (-200));
			}
			if (effectInfo.Knockback)
			{
				Direction directionOfFire = invoker.Position.DirectionTo (target);
				foreach (var enemy in state.AllCharacters.Where (x => areaAffected.Contains (x.Position)).ToList ())
				{
					Point knockbackTarget = enemy.Position.InDirection (directionOfFire);
					if (IsPointClear (state, knockbackTarget))
					{
						Animation.Request (state, new MovementAnimationInfo (enemy, knockbackTarget));
						state = state.WithReplaceCharacter (enemy.WithPosition (knockbackTarget));
					}
				}
			}
			return state;
		}

		GameState ChargeSkillForResources (GameState state, Character invoker, Skill skill)
		{
			if (skill.UsesAmmo)
				skill = skill.WithLessAmmo ();

			if (skill.UsesCooldown)
			{
				if (!skill.RechargedAmmoOnCooldown || !skill.UnderCooldown)
				{
					skill = skill.WithCooldownSet ();
					state = state.WithAddedScript (Generator.CreateCooldownScript (1, invoker, skill));
				}
			}

			return state.WithReplaceCharacter (invoker.WithReplaceSkill (skill));
		}

		public IEnumerable<Character> CharactersAffectedByMoveAndDamage (GameState state, Character invoker, Skill skill, Point position)
		{
			int range = ((MoveAndDamageSkillEffectInfo)skill.EffectInfo).Range;
			var orderedCharactersByDistance = state.AllCharacters.Select (x => new Tuple<double, Character> (position.GridDistance (x.Position), x));
			var potentialTargets = orderedCharactersByDistance.Where (x => x.Item1 <= range).OrderBy (x => x.Item1).Select (x => x.Item2);
			var targetsOfCorrectSide = potentialTargets.Where (x => x.ID != invoker.ID).Where (x => x.IsPlayer != invoker.IsPlayer); // #105
			return targetsOfCorrectSide.Where (x => IsPathBetweenPointsClear (state, position, x.Position, false));
		}

		public HashSet<Point> UnblockedPointsInBurst (GameState state, Point target, int area)
		{
			return new HashSet<Point> (target.PointsInBurst (area).Where (x => IsPathBetweenPointsClear (state, target, x, true)));
		}

		public HashSet<Point> UnblockedPointsNextToPoint (GameState state, Point target)
		{
			return new HashSet<Point> (target.PointsInBurst (1).Where (x => IsPathBetweenPointsClear (state, target, x, true)));
		}

		public HashSet<Point> UnblockedPointsInCone (GameState state, Character invoker, Skill skill, Point target)
		{
			Direction direction = invoker.Position.DirectionTo (target);
			return new HashSet<Point> (invoker.Position.PointsInCone (direction, skill.TargetInfo.Range).Where(x => IsPathBetweenPointsClear(state, invoker.Position, x, true)));
		}

		public HashSet<Point> UnblockedPointsInLine (GameState state, Character invoker, Skill skill, Point target)
		{
			Direction direction = invoker.Position.DirectionTo (target);
			Point lineTarget = invoker.Position;
			for (int i = 0; i < skill.TargetInfo.Range; ++i)
				lineTarget = lineTarget.InDirection (direction);

			return new HashSet<Point> (BresenhamLine.PointsOnLine (invoker.Position, lineTarget).Where (x => IsPathBetweenPointsClear (state, invoker.Position, x, true)));
		}

		public HashSet<Point> AffectedPointsForSkill (GameState state, Character invoker, Skill skill, Point target)
		{
			switch (skill.TargetInfo.TargettingStyle)
			{
				case TargettingStyle.Point:
					return UnblockedPointsInBurst (state, target, skill.TargetInfo.Area);
				case TargettingStyle.Cone:
					return UnblockedPointsInCone (state, invoker, skill, target);
				case TargettingStyle.Line:
					return UnblockedPointsInLine (state, invoker, skill, target);
				default:
					return new HashSet<Point> ();
			}
		}

		public bool IsValidTarget (GameState state, Character invoker, Skill skill, Point target)
		{
			switch (skill.Effect)
			{
				case Effect.Movement:
				case Effect.MoveAndDamageClosest:
					if (state.AllCharacters.Any (x => x.Position == target))
						return false;
					break;
			}

			switch (skill.TargetInfo.TargettingStyle)
			{
				case TargettingStyle.Point:
				{
					if (!state.Map.IsOnMap (target))
						return false;

					TargettingInfo targetInfo = skill.TargetInfo;
					Point source = invoker.Position;
					if (!SkillInRange (source, target, targetInfo))
						return false;

					MapVisibility visibility = state.CalculateVisibility (invoker);
					if (!visibility.IsVisible (target))
						return false;

					return IsPathBetweenPointsClear (state, invoker.Position, target, false);
				}
				case TargettingStyle.Cone:
				{
					if (!state.Map.IsOnMap (target))
						return false;

					if (!state.Map [target].Walkable)
						return false;

					if (SkillInRange (invoker.Position, target, skill.TargetInfo))
					{
						Direction direction = invoker.Position.DirectionTo (target);
						return direction == Direction.North || direction == Direction.South || direction == Direction.West || direction == Direction.East;
					}
					return false;
				}
				case TargettingStyle.Line:
				{
						Direction direction = invoker.Position.DirectionTo (target);
						if (direction == Direction.None)
							return false;
						return true;
				}
				default:
					return true;		
			}
		}

		static bool IsPathBetweenPointsClear (GameState state, Point source, Point target, bool pierceCharacters)
		{
			if (!state.Map.IsOnMap (target))
				return false;
			if (state.Map[target].Terrain != TerrainType.Floor)
				return false;

			foreach (Point p in BresenhamLine.PointsOnLine (source, target))
			{
				if (!state.Map.IsOnMap (p))
					return false;
				if (p == target)
					return true;
				if (!IsPointClear (state, p, pierceCharacters))
					return false;
			}
			return true;
		}

		static bool IsPointClear (GameState state, Point p, bool pierceCharacters = false)
		{
			if (state.Map [p].Terrain != TerrainType.Floor)
				return false;
			if (!pierceCharacters && state.AllCharacters.Any (x => x.Position == p))
				return false;
			return true;
		}

		static bool SkillInRange (Point source, Point target, TargettingInfo targetInfo)
		{
			switch (targetInfo.TargettingStyle)
			{
				case TargettingStyle.Point:
					return target.GridDistance (source) <= targetInfo.Range;
				case TargettingStyle.Cone:
					return target.LatticeDistance (source) == 1;
				case TargettingStyle.Line:
				case TargettingStyle.None:
				default:
					return true;
			}
		}

		public HashSet<Point> PointsSkillCanTarget (GameState state, Character invoker, Skill skill)
		{
			switch (skill.TargetInfo.TargettingStyle)
			{
				case TargettingStyle.Point:
				{
					var pointsInRange = UnblockedPointsInBurst (state, invoker.Position, skill.TargetInfo.Range + 1);
					var validPointsInRange = pointsInRange.Where (x => IsValidTarget (state, invoker, skill, x));
					return new HashSet<Point> (validPointsInRange);
				}
				case TargettingStyle.Cone:
				{
					var pointsInRange = UnblockedPointsNextToPoint (state, invoker.Position);
					var validPointsInRange = pointsInRange.Where (x => IsValidTarget (state, invoker, skill, x));
					return new HashSet<Point> (validPointsInRange);
				}
				case TargettingStyle.Line:
				case TargettingStyle.None:
				default:
					return null;
			}
		}
	}
}