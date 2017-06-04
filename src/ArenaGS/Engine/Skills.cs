using System;
using ArenaGS.Model;
using ArenaGS.Utilities;

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
					return Physics.Damage (state, invoker, 1);
				case Effect.None:
					break;
			}

			return Physics.WaitPlayer (state).WithNewLogLine ($"Skill: {skill.Name} at {target}");
		}
	}
}
