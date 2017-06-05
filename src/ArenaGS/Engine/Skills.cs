using System;

using ArenaGS.Model;
using ArenaGS.Utilities;
using System.Collections.Generic;
using System.Linq;
using ArenaGS.Engine.Utilities;

namespace ArenaGS.Engine
{
	public interface ISkills
	{
		GameState Invoke (GameState state, Character invoker, Skill skill, Point target);
		bool IsValidTarget (GameState state, Character invoker, Skill skill, Point target);
	}

	public class Skills : ISkills
	{
		IPhysics Physics;

		public Skills ()
		{
			Physics = Dependencies.Get<IPhysics> ();
		}

		public GameState Invoke (GameState state, Character invoker, Skill skill, Point target)
		{
			if (!invoker.Skills.Contains (skill))
				throw new InvalidOperationException ($"{invoker} tried to invoke {skill.Name} but did not contain it.");

			if (!IsValidTarget (state, invoker, skill, target))
				throw new InvalidOperationException ($"{invoker} tried to invoke {skill.Name} at {target} but was invalid.");

			switch (skill.Effect)
			{
				case Effect.Damage:
					HashSet<Point> areaAffected = new HashSet<Point> (target.PointsInBurst (skill.TargetInfo.Area));					
					foreach (var enemy in state.Enemies.Concat (state.Player.Yield ()).Where (x => areaAffected.Contains (x.Position)))
						state = Physics.Damage (state, enemy, 1);
					break;
				case Effect.None:
					break;
			}

			return Physics.Wait (state, invoker).WithNewLogLine ($"Skill: {skill.Name} at {target}");
		}

		public bool IsValidTarget (GameState state, Character invoker, Skill skill, Point target)
		{
			if (!state.Map.IsOnMap (target))
				return false;

			TargettingInfo targetInfo = skill.TargetInfo;
			Point source = invoker.Position;
			return SkillInRnage (source, target, targetInfo) && SkillPathIsClear (state, source, target, targetInfo);
		}

		static bool SkillPathIsClear (GameState state, Point source, Point target, TargettingInfo targetInfo)
		{
			foreach (Point p in BresenhamLine.PointsOnLine (source, target))
			{
				if (p == target)
					return true;
				if (state.Map[p].Terrain == TerrainType.Wall)
					return false;
				if (state.Enemies.Any (x => x.Position == p))
					return false;
			}
			return true;
		}

		static bool SkillInRnage (Point source, Point target, TargettingInfo targetInfo)
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
