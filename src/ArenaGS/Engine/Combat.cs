using ArenaGS.Model;
using ArenaGS.Platform;

namespace ArenaGS.Engine
{
	public interface ICombat
	{
		GameState Damage (GameState state, Character target, int amount);
	}

	class Combat : ICombat
	{
		IRandomGenerator Random;
		IAnimationRequest Animation;

		public Combat ()
		{
			Random = Dependencies.Get<IRandomGenerator> ();
			Animation = Dependencies.Get<IAnimationRequest> ();
		}

		public GameState Damage (GameState state, Character target, int amount)
		{
			int damageDiceTotal = amount - target.Defense.StandardDefense;
			if (damageDiceTotal <= 0)
				return state;

			Dice damageRoll = new Dice (damageDiceTotal, 3, damageDiceTotal);
			int actualDamage = damageRoll.Roll (Random);
			int remainingHealth = target.Health.Current - actualDamage;
			if (remainingHealth > 0)
			{
				return state.WithReplaceCharacter (target.WithHealth (target.Health.WithCurrentHealth (remainingHealth))).WithNewLogLine ($"{target} damaged by {actualDamage}.");
			}
			else
			{
				if (target.IsPlayer)
				{
					GameState deathState = state.WithReplaceCharacter (target.WithHealth (target.Health.WithCurrentHealth (remainingHealth))).WithNewLogLine ($"{target} damaged by {actualDamage}.");
					Animation.RequestPlayerDead (deathState);
					return deathState;
				}
				else
				{
					return state.WithRemovedEnemy (target).WithNewLogLine ($"{target} killed.");
				}
			}
		}
	}
}
