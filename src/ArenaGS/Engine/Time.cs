using System.Linq;
using ArenaGS.Model;
using System.Collections.Immutable;
using System;

namespace ArenaGS.Engine
{
	public interface ITimedElement
	{
		int CT { get; }
	}

	public interface ITime
	{
		GameState ProcessUntilPlayerReady (GameState state);
		int ChargeTime (ITimedElement c, int amount);
	}

	// Minor hack
	internal static class TimeConstants
	{
		internal const int CTNededForAction = 100;
		internal const int CTPerTick = 10;
		internal const int CTPerMovement = 100;
		internal const int CTPerBasicAction = 100;
	}

	public class Time : ITime
	{
		IActorBehavior ActorBehavior;
		IScriptBehavior ScriptBehavior;

		public Time ()
		{
		}

		// We can not do this in Time constructor as we have a circular dependency
		// Time uses behaviors to process, and they may need Time to act
		public void GetDependenciesIfNeeded ()
		{
			if (ActorBehavior == null)
				ActorBehavior = Dependencies.Get<IActorBehavior> ();
			if (ScriptBehavior == null)
				ScriptBehavior = Dependencies.Get<IScriptBehavior> ();
		}

		public int ChargeTime (ITimedElement c, int amount)
		{
			if (c.CT < amount)
				throw new InvalidOperationException ($"{c} tried to act requring {amount} but only had {c.CT}");
			return c.CT - amount;
		}

		public GameState ProcessUntilPlayerReady (GameState state)
		{
			GetDependenciesIfNeeded ();

			// For as long as it takes
			while (true)
			{
				ITimedElement next = state.AllActors.FirstOrDefault (x => x.CT >= TimeConstants.CTNededForAction);
				
				if (next != null)
				{
					// TODO C#7
					if (next is Character activeCharacter) 
					{
						if (activeCharacter.IsPlayer)
							return state;
						else
							state = ActorBehavior.Act (state, activeCharacter);
					}
					else if (next is MapScript activeScript) 
					{
						state = ScriptBehavior.Act (state, activeScript);
					}
					else 
					{
						throw new NotImplementedException ();
					}
				}
				else // Else increment everyone's CT by the amount needed (int CTPerTick chunks) 
				{
					int ctNeeded = TimeConstants.CTNededForAction - state.AllActors.OrderByDescending (x => x.CT).First ().CT;
					int ticksNeeded = ctNeeded / 10;
					if ((ctNeeded % 10) > 0)
						ticksNeeded += 1;
					
					int additionalTicks = ticksNeeded * TimeConstants.CTPerTick;
					state = state.WithPlayer (state.Player.WithAdditionalCT (additionalTicks));
					state = state.WithEnemies (state.Enemies.Select (x => x.WithAdditionalCT (additionalTicks)).ToImmutableList ());
					state = state.WithScripts (state.Scripts.Select (x => x.WithAdditionalCT (additionalTicks)).ToImmutableList ());
				}
			}
		}
	}
}
