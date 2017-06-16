﻿using ArenaGS.Utilities;

namespace ArenaGS
{
	public enum Command
	{
		NewGame, // Null
		PlayerMove,  // Direction
		Wait, // Null
		Skill, // SkillTarget
	};

	public struct SkillTarget
	{
		public int Index;
		public Point Position; // Point.Empty if not needed
	}
}
