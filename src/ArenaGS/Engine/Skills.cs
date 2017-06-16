using System;
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
	}

	public class Skills : ISkills
	{
		IPhysics Physics;
		IAnimationRequest Animation;
		IGenerator Generator;
	
		public Skills ()
		{
			Physics = Dependencies.Get<IPhysics> ();
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
					HashSet<Point> areaAffected = AffectedPointsForSkill (state, invoker, skill, target);

					switch (skill.TargetInfo.TargettingStyle)
					{
							case TargettingStyle.Point:
							{
								List<Point> path = BresenhamLine.PointsOnLine (invoker.Position, target);
								Animation.Request (state, new ProjectileAnimationInfo (path));

								if (skill.TargetInfo.Area > 1)
									Animation.Request (state, new ExplosionAnimationInfo (target, skill.TargetInfo.Area, areaAffected.ToImmutableHashSet ()));

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
								Animation.Request(state, new SpecificAreaExplosionAnimationInfo (areaAffected.ToImmutableHashSet ()));
								break;
							}
					}

					foreach (var enemy in state.AllCharacters.Where (x => areaAffected.Contains (x.Position)))
						state = Physics.Damage (state, enemy, 1);

					break;
				}
				case Effect.DelayedDamage:
				{
					HashSet<Point> areaAffected = AffectedPointsForSkill (state, invoker, skill, target);
					state = state.WithAddedScript (Generator.CreateDamageScript (-100, 1, areaAffected.ToImmutableHashSet ()));
					break;
				}
				case Effect.None:
					break;
			}

			invoker = state.UpdateCharacterReference (invoker);
			state = ChargeSkillForResources (state, invoker, skill);

			return Physics.Wait (state, invoker).WithNewLogLine ($"Skill: {skill.Name} at {target}");
		}

		GameState ChargeSkillForResources (GameState state, Character invoker, Skill skill)
		{
			if (skill.UsesAmmo)
				skill = skill.WithLessAmmo ();

			if (skill.UsesCooldown)
			{
				skill = skill.WithCooldownSet ();
				state = state.WithAddedScript (Generator.CreateCooldownScript (1, invoker, skill));
			}

			return state.WithReplaceCharacter (invoker.WithReplaceSkill (skill));
		}

		public HashSet<Point> UnblockedPointsInBurst (GameState state, Skill skill, Point target)
		{
			return new HashSet<Point> (target.PointsInBurst (skill.TargetInfo.Area).Where (x => IsPathBetweenPointsClear (state, target, x, true)));
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
					return UnblockedPointsInBurst (state, skill, target);
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

					int distance = invoker.Position.LatticeDistance (target);
					if (distance == 1)
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
				if (state.Map[p].Terrain != TerrainType.Floor)
					return false;
				if (!pierceCharacters && state.AllCharacters.Any (x => x.Position == p))
					return false;
			}
			return true;
		}

		static bool SkillInRange (Point source, Point target, TargettingInfo targetInfo)
		{
			switch (targetInfo.TargettingStyle)
			{
				case TargettingStyle.Point:
					return target.NormalDistance (source) <= targetInfo.Range;
				case TargettingStyle.None:
				default:
					return true;
			}
		}
	}
}
