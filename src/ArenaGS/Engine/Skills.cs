using System;

using ArenaGS.Model;
using ArenaGS.Utilities;
using System.Collections.Generic;
using System.Linq;
using ArenaGS.Engine.Utilities;
using ArenaGS.Platform;

namespace ArenaGS.Engine
{
	public interface ISkills
	{
		GameState Invoke (GameState state, Character invoker, Skill skill, Point target);
		bool IsValidTarget (GameState state, Character invoker, Skill skill, Point target);
		HashSet<Point> UnblockedPointsInBurst (GameState state, Skill skill, Point target);
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
					List<Point> path = BresenhamLine.PointsOnLine (invoker.Position, target);
					Animation.Request (state, new ProjectileAnimationInfo (AnimationType.Projectile, path));

					HashSet<Point> areaAffected = UnblockedPointsInBurst (state, skill, target);

					if (skill.TargetInfo.Area > 1)
						Animation.Request (state, new ExplosionAnimationInfo (target, skill.TargetInfo.Area, areaAffected));

					foreach (var enemy in state.AllCharacters.Where (x => areaAffected.Contains (x.Position)))
						state = Physics.Damage (state, enemy, 1);

					invoker = state.UpdateCharacterReference (invoker);
					break;
				}
				case Effect.None:
					break;
			}

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
				state = state.WithScripts (state.Scripts.Add (Generator.CreateCooldownScript (1, invoker, skill)));
			}

			return state.WithReplaceCharacter (invoker.WithReplaceSkill (skill));
		}

		public HashSet<Point> UnblockedPointsInBurst (GameState state, Skill skill, Point target)
		{
			return new HashSet<Point> (target.PointsInBurst (skill.TargetInfo.Area).Where (x => IsPathBetweenPointsClear (state, target, x)));
		}

		public bool IsValidTarget (GameState state, Character invoker, Skill skill, Point target)
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

			return IsPathBetweenPointsClear (state, invoker.Position, target);
		}

		static bool IsPathBetweenPointsClear (GameState state, Point source, Point target)
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
				if (state.AllCharacters.Any (x => x.Position == p))
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
