using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Engine
{
	static class Physics
	{
		internal static GameState MovePlayer (GameState state, Direction direction)
		{
			return state.WithPlayer (MoveCharacter (state, state.Player, direction));
		}

		internal static GameState WaitPlayer (GameState state)
		{
			return state.WithPlayer (Wait (state.Player));
		}

		internal static GameState MoveEnemy (GameState state, Character enemy, Direction direction)
		{
			return state.WithReplaceEnemy (MoveCharacter (state, enemy, direction));
		}

		internal static GameState WaitEnemy (GameState state, Character enemy)
		{
			return state.WithReplaceEnemy (Wait (enemy));
		}

		internal static bool CouldCharacterWalk (GameState state, Character actor, Point newPosition)
		{
			Map map = state.Map;

			if (!map.IsOnMap (newPosition))
				return false;

			bool isWalkableLocation = map[newPosition].Terrain == TerrainType.Floor;
			bool isLocationEmpty = state.AllActors.All (x => x.Position != newPosition);
			return isWalkableLocation && isLocationEmpty;
		}

		static Character MoveCharacter (GameState state, Character actor, Direction direction)
		{
			Point newPosition = actor.Position.InDirection (direction);
			if (CouldCharacterWalk (state, actor, newPosition))
				return (actor.WithPosition (newPosition, actor.CT - Time.CTPerMovement));
			return actor;
		}

		internal static Character Wait (Character c)
		{
			return c.WithCT (c.CT - Time.CTPerBasicAction);
		}
	}
}
