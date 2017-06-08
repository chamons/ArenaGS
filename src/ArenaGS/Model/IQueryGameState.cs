using System;
using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;

namespace ArenaGS
{
	public interface IQueryGameState
	{
		bool IsValidTargetForSkill (GameState state, Skill skill, Point target);
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
	}
}
