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
			if (map.IsOnMap (newPosition) && map[newPosition].Terrain == TerrainType.Floor)
				return state.WithNewPlayer (actor.WithNewPosition (newPosition));
			return state;
		}
	}
}
