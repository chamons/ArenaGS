using System.Linq;

using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;

namespace ArenaGS.Engine
{
	public interface IPhysics
	{
		GameState MovePlayer (GameState state, Direction direction);
		GameState MoveEnemy (GameState state, Character enemy, Direction direction);

		GameState WaitPlayer (GameState state);
		GameState WaitEnemy (GameState state, Character enemy);
		GameState Wait (GameState state, Character c);
		GameState Stun (GameState state, Character c);
		GameState KnockBack (GameState state, Character c, Direction direction);

		bool IsPointClear (GameState state, Point p, bool pierceCharacters = false);
		bool CouldCharacterWalk (GameState state, Character actor, Point newPosition);		
	}

	public class Physics : IPhysics
	{
		ILogger Log;
		ITime Time;
		IAnimationRequest Animation;

		public Physics ()
		{
			Time = Dependencies.Get<ITime> ();
			Log = Dependencies.Get<ILogger> ();
			Animation = Dependencies.Get<IAnimationRequest> ();
		}
		
		public GameState MovePlayer (GameState state, Direction direction)
		{
			return state.WithPlayer (MoveCharacter (state, state.Player, direction));
		}

		public GameState WaitPlayer (GameState state)
		{
			return state.WithPlayer (Wait (state.Player));
		}

		public GameState MoveEnemy (GameState state, Character enemy, Direction direction)
		{
			return state.WithReplaceEnemy (MoveCharacter (state, enemy, direction));
		}

		public GameState WaitEnemy (GameState state, Character enemy)
		{
			return state.WithReplaceEnemy (Wait (enemy));
		}


		const string StunResistanceName = "Stun";
		const int BaseStunValue = -150;
		public GameState Stun (GameState state, Character c)
		{
			var newResistance = c.EffectResistance.WithResistanceIncremented (StunResistanceName);
			state = state.WithReplaceCharacter (c.WithEffectResistance (newResistance));
			c = state.UpdateCharacterReference (c);

			return state.WithReplaceCharacter (c.WithAdditionalCT (GetStunValue (newResistance)));
		}

		int GetStunValue (EffectResistance newResistance)
		{
			switch (newResistance[StunResistanceName])
			{
				case 1:
					return BaseStunValue;
				case 2:
					return BaseStunValue / 3;
				case 3:
				default:
					return 0;
			}
		}

		public GameState KnockBack (GameState state, Character c, Direction direction)
		{
			Point knockbackTarget = c.Position.InDirection (direction);
			if (IsPointClear (state, knockbackTarget))
			{
				Animation.Request (state, new MovementAnimationInfo (c, knockbackTarget));
				state = state.WithReplaceCharacter (c.WithPosition (knockbackTarget));
			}
			return state;
		}

		public bool IsPointClear (GameState state, Point p, bool pierceCharacters = false)
		{
			if (state.Map [p].Terrain != TerrainType.Floor)
				return false;
			if (!pierceCharacters && state.AllCharacters.Any (x => x.Position == p))
				return false;
			return true;
		}

		public GameState Wait (GameState state, Character c)
		{
			if (c.IsPlayer)
				return WaitPlayer (state);
			else
				return WaitEnemy (state, c); 
		}

		public bool CouldCharacterWalk (GameState state, Character actor, Point newPosition)
		{
			Map map = state.Map;

			if (!map.IsOnMap (newPosition))
				return false;

			bool isWalkableLocation = map[newPosition].Terrain == TerrainType.Floor;
			bool isLocationEmpty = state.AllCharacters.All (x => x.Position != newPosition);
			return isWalkableLocation && isLocationEmpty;
		}

		Character MoveCharacter (GameState state, Character actor, Direction direction)
		{
			Point newPosition = actor.Position.InDirection (direction);
			if (CouldCharacterWalk (state, actor, newPosition))
			{
				if (!actor.IsPlayer)
				{
					Log.Log (() => $"{actor} in {direction} to {newPosition}", LogMask.Animation);
					Animation.Request (state, new MovementAnimationInfo (actor, newPosition));
				}
				return actor.WithPosition (newPosition, Time.ChargeTime (actor, TimeConstants.CTPerMovement));
			}

			return actor;
		}

		Character Wait (Character c)
		{
			return c.WithCT (Time.ChargeTime (c, TimeConstants.CTPerBasicAction));
		}
	}
}
