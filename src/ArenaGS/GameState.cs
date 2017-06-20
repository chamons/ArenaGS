using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

using ArenaGS.Engine;
using ArenaGS.Engine.Utilities;
using ArenaGS.Model;

using ProtoBuf;
using ArenaGS.Utilities;

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

		internal GameState WithAddedEnemy (Character enemy)
		{
			return new GameState (this) { Enemies = Enemies.Add (enemy) };
		}

		internal GameState WithRemovedEnemy (Character enemy)
		{
			return new GameState (this) { Enemies = Enemies.Remove (enemy) };
		}

		internal GameState WithEnemies (ImmutableList<Character> enemies)
		{
			return new GameState (this) { Enemies = enemies };
		}

		internal GameState WithActors (IEnumerable<ITimedElement> actors)
		{
			Character player = actors.OfType<Character> ().FirstOrDefault (x => x.IsPlayer);

			GameState newState = new GameState (this)
			{
				Enemies = actors.OfType<Character> ().Where (x => !x.IsPlayer).ToImmutableList (),
				Scripts = actors.OfType<MapScript> ().ToImmutableList ()
			};

			if (player != null)
				newState.Player = (player);
			return newState;
		}

		internal GameState WithCharacters (IEnumerable<Character> characters)
		{
			Character player = characters.FirstOrDefault (x => x.IsPlayer);
			GameState newState = new GameState (this) { Enemies = characters.Where (x => !x.IsPlayer).ToImmutableList ()  };
			if (player != null)
				newState.Player = (player);
			return newState;
		}

		internal GameState WithAddedScript (MapScript newScript)
		{
			return new GameState (this) { Scripts = Scripts.Add (newScript) };
		}

		internal GameState WithRemovedScript (MapScript removedScript)
		{
			return new GameState (this) { Scripts = Scripts.Remove (removedScript) };
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
			return new GameState (this) { LogEntries = logBuilder.ToImmutable () };
		}

		internal GameState WithReplaceEnemy (Character newEnemy)
		{
			Character oldEnemy = Enemies.First (x => x.ID == newEnemy.ID);
			return new GameState (this) { Enemies = Enemies.Replace (oldEnemy, newEnemy) };
		}

		internal GameState WithReplaceScript (MapScript newScript)
		{
			MapScript oldScript = Scripts.First (x => x.ID == newScript.ID);
			return new GameState (this) { Scripts = Scripts.Replace (oldScript, newScript) };
		}

		internal GameState WithReplaceCharacter (Character character)
		{
			if (character.IsPlayer)
				return WithPlayer (character);
			else
				return WithReplaceEnemy (character);
		}

		// TODO - This cache could be copied in GameState (GameState original) if we are 
		// very careful about invalidation.
		int [,] shortestPath;
		internal int [,] ShortestPath
		{
			get
			{
				if (shortestPath == null)
					shortestPath = Dijkstra.CalculateShortestPathArray (Map, Player.Position);
				return shortestPath;
			}
		}

		Dictionary<Character, MapVisibility> VisibiltyCache;
		public MapVisibility CalculateVisibility (Character c)
		{
			if (VisibiltyCache == null)
				VisibiltyCache = new Dictionary<Character, MapVisibility> ();

			MapVisibility visibility;
			if (VisibiltyCache.TryGetValue (c, out visibility))
				return visibility;

			visibility = ShadowCastingFOV.ComputeRecursiveShadowcasting (this, c, 100, true);
			VisibiltyCache[c] = visibility;
			return visibility;
		}

		public Character UpdateCharacterReference (Character oldReference)
		{
			return AllCharacters.First (x => x.ID == oldReference.ID);
		}

		public MapScript UpdateScriptReference (MapScript oldReference)
		{
			return Scripts.First (x => x.ID == oldReference.ID);
		}

		public MapScript UpdateScriptReferenceIfExists (MapScript oldReference)
		{
			return Scripts.FirstOrDefault (x => x.ID == oldReference.ID);
		}
	}
}
