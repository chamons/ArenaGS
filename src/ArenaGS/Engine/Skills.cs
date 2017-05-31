﻿using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Engine
{
	static class Skills
	{
		// TODO tests
		// TODO - validation on target is in range
		internal static GameState Invoke (GameState state, Character invoker, Skill skill, Point target)
		{
			switch (skill.Effect)
			{
				case Effect.Damage:					
				case Effect.None:
					break;
			}

			return Physics.WaitPlayer (state).WithNewLogLine ($"Skill: {skill.Name} at {target}");
		}
	}
}
