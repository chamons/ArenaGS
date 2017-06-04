using System;
using ArenaGS.Model;
using ArenaGS.Utilities;
using System.Collections.Generic;
using System.Linq;

namespace ArenaGS.Engine
{
	public interface ISkills
	{
		GameState Invoke (GameState state, Character invoker, Skill skill, Point target);
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

			if (!skill.TargetInfo.IsValidTarget (invoker.Position, target))
				throw new InvalidOperationException ($"{invoker} tried to invoke {skill.Name} at {target} but was invalid.");

			// Skill is in range of target

			switch (skill.Effect)
			{
				case Effect.Damage:
					HashSet<Point> areaAffected = new HashSet<Point> (target.PointsInBurst (skill.TargetInfo.Area));					
					foreach (var enemy in state.Enemies.Concat (state.Player.Yield ()).Where (x => areaAffected.Contains (x.Position)))
						state = Physics.Damage (state, enemy, 1);
					return state;
				case Effect.None:
					break;
			}

			return Physics.WaitPlayer (state).WithNewLogLine ($"Skill: {skill.Name} at {target}");
		}
	}
}
