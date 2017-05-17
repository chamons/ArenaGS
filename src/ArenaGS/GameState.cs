﻿using ArenaGS.Model;
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

		public GameState ()
		{
		}

		public GameState (Map map, Character player)
		{
			Map = map;
			Player = player;
		}

		GameState (GameState original)
		{
			Map = original.Map;
			Player = original.Player;
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
