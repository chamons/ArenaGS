using System.Linq;
using ArenaGS.Model;
using System.Collections.Immutable;

namespace ArenaGS.Engine
{
	internal static class Time
	{
		internal const int CTNededForAction = 100;
		internal const int CTPerTick = 10;
		internal const int CTPerMovement = 100;
		internal const int CTPerBasicAction = 100;

		internal static GameState ProcessUntilPlayerReady (GameState state)
		{
			IActorBehavior actorBehavior = Dependencies.Get<IActorBehavior> ();

			// For as long as it takes
			while (true)
			{
				// Find the next actor to run
				Character next = state.AllActors.OrderBy (x => x.CT).Reverse ().First ();

				// If they have enough CT, act
				if (next.CT >= CTNededForAction)
				{
					// If it is the player, we're done
					if (next == state.Player)
						return state;

					state = actorBehavior.Act (state, next);
				}
				else // Else increment everyone's CT by the amount needed (int CTPerTick chunks) 
				{
					int ctNeeded = CTNededForAction - next.CT;
					int ticksNeeded = ctNeeded % 10;
					if ((ctNeeded / 10) > 0)
						ticksNeeded += 1;

					state = state.WithPlayer (state.Player.WithAdditionalCT (ticksNeeded * CTPerTick));
					state = state.WithEnemies (state.Enemies.Select (x => x.WithAdditionalCT (ticksNeeded * CTPerTick)).ToImmutableList ());
				}
			}
		}
	}
}
