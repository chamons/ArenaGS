using System.Collections.Immutable;
using ArenaGS.Model;
using ProtoBuf;

namespace ArenaGS
{
	[ProtoContract]
	public sealed class GameState
	{
		[ProtoMember (1)]
		public Map Map { get; private set; }

		[ProtoMember (2)]
		public Character Player { get; private set; }

		[ProtoMember (3)]
		public ImmutableList<Character> Enemies { get; private set; }

		[ProtoMember (4)]
		public ImmutableList<string> LogEntries { get; private set; }

		public GameState ()
		{
		}

		public GameState (Map map, Character player, ImmutableList<Character> enemies, ImmutableList<string> logEntries)
		{
			Map = map;
			Player = player;
			Enemies = enemies;
			LogEntries = logEntries;
		}

		GameState (GameState original)
		{
			Map = original.Map;
			Player = original.Player;
			Enemies = original.Enemies;
			LogEntries = original.LogEntries;
		}

		internal GameState WithPlayer (Character player)
		{
			return new GameState (this) { Player = player };
		}

		internal GameState WithMap (Map map)
		{
			return new GameState (this) { Map = map };
		}

		internal GameState WithEnemies (ImmutableList<Character> enemies)
		{
			return new GameState (this) { Enemies = enemies };
		}

		internal GameState WithLog (ImmutableList<string> logEntries)
		{
			return new GameState (this) { LogEntries = logEntries };
		}

		internal GameState WithNewLogLine (string line)
		{
			var logBuilder = LogEntries.ToBuilder ();
			if (logBuilder.Count == 5)
				logBuilder.RemoveAt (0);
			logBuilder.Add (line);
			return new GameState (this) { LogEntries = logBuilder.ToImmutable() };
		}
	}
}
