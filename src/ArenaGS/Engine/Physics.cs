using System.Linq;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Engine
{
	static class Physics
	{
		internal static GameState Move (Character actor, Direction direction, GameState state)
		{
			Map map = state.Map;
			Point newPosition = actor.Position.InDirection (direction);

			if (!map.IsOnMap (newPosition))
				return state;

			bool isWalkableLocation = map[newPosition].Terrain == TerrainType.Floor;
			bool isLocationEmpty = state.Enemies.All (x => x.Position != newPosition);
			if (isWalkableLocation && isLocationEmpty)
				return state.WithPlayer (actor.WithPosition (newPosition));
			return state;
		}
	}
}
