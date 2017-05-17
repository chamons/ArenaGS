using System.Collections.Immutable;
using ArenaGS.Model;
using ProtoBuf;

namespace ArenaGS
{
	[ProtoContract]
	public class GameState
	{
		[ProtoMember (1)]
		public Map Map { get; private set; }

		[ProtoMember (2)]
		public Character Player { get; private set; }

		[ProtoMember (3)]
		public ImmutableList<Character> Enemies { get; private set; }

		public GameState ()
		{
		}

		public GameState (Map map, Character player, ImmutableList<Character> enemies)
		{
			Map = map;
			Player = player;
			Enemies = enemies;
		}

		GameState (GameState original)
		{
			Map = original.Map;
			Player = original.Player;
			Enemies = original.Enemies;
		}

		internal GameState WithNewPlayer (Character player)
		{
			return new GameState (this) { Player = player };
		}

		internal GameState WithNewMap (Map map)
		{
			return new GameState (this) { Map = map };
		}
	}
}
