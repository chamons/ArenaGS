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
		ReduceCooldownScript CreateCooldownScript (int ct, Character character, Skill skill);

		Skill CreateSkill (string name, Effect effect, TargettingInfo targetInfo, SkillResources resources);
	}

	public class Generator : IGenerator
	{
		const int CharacterOffset = 100;
		const int ScriptOffset = 2000;
		const int SkillOffset = 30000;

		int CharacterCount = 0;
		int ScriptCount = 0;
		int SkillCount = 0;

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

		int NextSkillID ()
		{
			int next = SkillOffset + SkillCount;
			SkillCount++;
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
				CreateSkill ("Fireball", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 8), new SkillResources (maxCooldown : 3)),
				CreateSkill ("Grenade", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 4, 3), new SkillResources (maxAmmo : 2))
			}.ToImmutableList ());
		}

		public ImmutableList<Character> CreateCharacters (IEnumerable<Point> positions)
		{
			return positions.Select (x => CreateCharacter (x)).ToImmutableList();
		}

		public SpawnerScript CreateSpawner (Point position)
		{
			return new SpawnerScript (NextScriptID (), position, 100, 5, 3);
		}

		public Skill CreateSkill (string name, Effect effect, TargettingInfo targetInfo, SkillResources resources)
		{
			return new Skill (NextSkillID (), name, effect, targetInfo, resources);
		}

		public ReduceCooldownScript CreateCooldownScript (int ct, Character character, Skill skill)
		{
			return new ReduceCooldownScript (NextScriptID (), ct, character.ID, skill.ID);
		}
	}
}