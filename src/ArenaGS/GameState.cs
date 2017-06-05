using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

using ArenaGS.Engine;
using ArenaGS.Engine.Utilities;
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

		[ProtoMember (5)]
		public ImmutableList<MapScript> Scripts { get; private set; }

		public IEnumerable<Character> AllCharacters 
		{
			get 
			{
				yield return Player;
				foreach (var enemy in Enemies)
					yield return enemy;
			}
		}

		public IEnumerable<ITimedElement> AllActors
		{
			get
			{
				yield return Player;
				foreach (var enemy in Enemies)
					yield return enemy;

				foreach (var script in Scripts)
					yield return script;
			}
		}

		public GameState ()
		{
		}

		public GameState (Map map, Character player, ImmutableList<Character> enemies, ImmutableList<MapScript> scripts, ImmutableList<string> logEntries)
		{
			Map = map;
			Player = player;
			Enemies = enemies;
			Scripts = scripts;
			LogEntries = logEntries;
		}

		GameState (GameState original)
		{
			Map = original.Map;
			Player = original.Player;
			Enemies = original.Enemies;
			Scripts = original.Scripts;
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

		internal GameState WithScripts (ImmutableList<MapScript> scripts)
		{
			return new GameState (this) { Scripts = scripts };
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

		internal GameState WithReplaceEnemy (Character newEnemy)
		{
			Character oldEnemy = Enemies.Single (x => x.ID == newEnemy.ID);
			return new GameState (this) { Enemies = Enemies.Replace (oldEnemy, newEnemy) };
		}

		internal GameState WithReplaceScript (MapScript newScript)
		{
			MapScript oldScript = Scripts.Single (x => x.ID == newScript.ID);
			return new GameState (this) { Scripts = Scripts.Replace (oldScript, newScript) };
		}

		// TODO - This cache could be copied in GameState (GameState original) if we are 
		// very careful about invalidation.
		private int[,] shortestPath;
		internal int[,] ShortestPath
		{
			get
			{
				if (shortestPath == null)
					shortestPath = Dijkstra.CalculateShortestPathArray (Map, Player.Position);
				return shortestPath;
			}
		}
	}
}
