using System;
using System.Collections.Generic;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;
using ArenaGS.Engine.Utilities;

namespace ArenaGS
{
	public interface IQueryGameState
	{
		bool IsValidTargetForSkill (GameState state, Skill skill, Point target);
		HashSet<Point> AffectedPointsForSkill (GameState state, Skill skill, Point target);

		bool HasSecondaryPointsForSkill (Skill skill);
		HashSet<Point> AffectedSecondaryPointsForSkill (GameState state, Skill skill, Point target);

		HashSet<Point> PointsSkillCanTarget (GameState state, Skill skill);
	}

	// Non-mutation calcuation requests on the current GameState
	public class QueryGameState : IQueryGameState
	{
		IPhysics Physics;
		ISkills Skills;

		public QueryGameState ()
		{
			Physics = Dependencies.Get<IPhysics> ();
			Skills = Dependencies.Get<ISkills> ();
		}

		public bool IsValidTargetForSkill (GameState state, Skill skill, Point target)
		{
			return Skills.IsValidTarget (state, state.Player, skill, target);
		}

		public HashSet <Point> AffectedPointsForSkill (GameState state, Skill skill, Point target)
		{
			return Skills.AffectedPointsForSkill (state, state.Player, skill, target);
		}

		public bool HasSecondaryPointsForSkill (Skill skill)
		{
			switch (skill.Effect)
			{
				case Effect.MoveAndDamageClosest:
					return true;
				default:
					return false;
			}
		}

		public HashSet<Point> AffectedSecondaryPointsForSkill (GameState state, Skill skill, Point target)
		{
			switch (skill.Effect)
			{
				case Effect.MoveAndDamageClosest:
				{
					MoveAndDamageSkillEffectInfo skillInfo = (MoveAndDamageSkillEffectInfo)skill.EffectInfo;
					return new HashSet<Point> (target.PointsInBurst (skillInfo.Range));
				}
				default:
					return null;
			}
		}

		public HashSet<Point> PointsSkillCanTarget (GameState state, Skill skill)
		{
			return Skills.PointsSkillCanTarget (state, state.Player, skill, state.Player.Position);
		}
	}
}
