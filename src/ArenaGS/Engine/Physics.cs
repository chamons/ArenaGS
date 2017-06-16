﻿using System.Linq;

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
