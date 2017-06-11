using System;
using System.Linq;
using System.Collections.Generic;
using System.Collections.Immutable;
using ArenaGS.Model;
using ArenaGS.Utilities;

namespace ArenaGS.Engine
{
	public interface IGenerator
	{
		GameState CreateEnemy (GameState state, Point position);
		Character CreateCharacter (Point position);
		Character CreatePlayer (Point position);
		ImmutableList<Character> CreateCharacters (IEnumerable<Point> positions);

		SpawnerScript CreateSpawner (Point position);
	}

	public class Generator : IGenerator
	{
		const int CharacterOffset = 100;
		const int ScriptOffset = 2000;
		int CharacterCount = 0;
		int ScriptCount = 0;

		int NextCharacterID ()
		{
			int next = CharacterOffset + CharacterCount;
			CharacterCount++;
			return next;
		}

		int NextScriptID  ()
		{
			int next = ScriptOffset + ScriptCount;
			ScriptCount++;
			return next;
		}

		public GameState CreateEnemy (GameState state, Point position)
		{
			return state.WithEnemies (state.Enemies.Add (CreateCharacter (position)));
		}

		public Character CreateCharacter (Point position)
		{
			return new Character (NextCharacterID (), position, 100, ImmutableList<Skill>.Empty);
		}

		public Character CreatePlayer (Point position)
		{
			return new Character (Character.PlayerID, position, 100, new Skill [] {
				new Skill ("Fireball", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 8), new SkillResources (maxCooldown : 2)),
				new Skill ("Grenade", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 4, 3), new SkillResources (maxAmmo : 2))
			}.ToImmutableList ());
		}

		public ImmutableList<Character> CreateCharacters (IEnumerable<Point> positions)
		{
			return positions.Select (x => CreateCharacter (x)).ToImmutableList();
		}

		public SpawnerScript CreateSpawner (Point position)
		{
			return new SpawnerScript (position, NextScriptID (), 100, 5, 3);
		}

	}
}