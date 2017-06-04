using System;
using System.Linq;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Engine
{
	public interface IPhysics
	{
		GameState MovePlayer (GameState state, Direction direction);
		GameState WaitPlayer (GameState state);
		GameState MoveEnemy (GameState state, Character enemy, Direction direction);
		GameState WaitEnemy (GameState state, Character enemy);
		GameState Damage (GameState state, Character target, int amount);
		Character Wait (Character c);

		bool CouldCharacterWalk (GameState state, Character actor, Point newPosition);		
	}

	public class Physics : IPhysics
	{
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

		public bool CouldCharacterWalk (GameState state, Character actor, Point newPosition)
		{
			Map map = state.Map;

			if (!map.IsOnMap (newPosition))
				return false;

			bool isWalkableLocation = map[newPosition].Terrain == TerrainType.Floor;
			bool isLocationEmpty = state.AllActors.All (x => x.Position != newPosition);
			return isWalkableLocation && isLocationEmpty;
		}

		Character MoveCharacter (GameState state, Character actor, Direction direction)
		{
			Point newPosition = actor.Position.InDirection (direction);
			if (CouldCharacterWalk (state, actor, newPosition))
				return actor.WithPosition (newPosition, ChargeTime (actor, Time.CTPerMovement));

			return actor;
		}

		public Character Wait (Character c)
		{
			return c.WithCT (ChargeTime (c, Time.CTPerBasicAction));
		}

		public int ChargeTime (Character c, int amount)
		{
			if (c.CT < amount)
				throw new InvalidOperationException ($"Character {c} tried to act requring {amount} but only had {c.CT}");
			return c.CT - amount;
		}

		public GameState Damage (GameState state, Character target, int amount)
		{
			return state.WithNewLogLine ($"{target} damaged by {amount}.");
		}
	}
}
