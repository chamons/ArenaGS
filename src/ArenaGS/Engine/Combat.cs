using ArenaGS.Model;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ArenaGS.Engine
{
	public interface ICombat
	{
		// TODO - https://github.com/chamons/ArenaGS/issues/79
		GameState Damage (GameState state, Character target, int amount);

	}

	class Combat : ICombat
	{
		// TODO - https://github.com/chamons/ArenaGS/issues/79
		public GameState Damage (GameState state, Character target, int amount)
		{
			if (target.IsPlayer)
				return state.WithNewLogLine ($"{target} damaged by {amount}.");
			else
				return state.WithEnemies (state.Enemies.Remove (target));
		}
	}
}
