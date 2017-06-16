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
		Character CreateCharacter (Point position, Health health, Defense defense);
		Character CreatePlayer (Point position, Health health, Defense defense);

		SpawnerScript CreateSpawner (Point position);
		ReduceCooldownScript CreateCooldownScript (int ct, Character character, Skill skill);
		AreaDamageScript CreateDamageScript (int ct, int damage, ImmutableHashSet<Point> affectedPoints);

		Skill CreateSkill (string name, Effect effect, TargettingInfo targetInfo, SkillResources resources, int power);

		Character CreateStubPlayer (Point position);
		Character CreateStubEnemy (Point position);
		ImmutableList<Character> CreateStubEnemies (IEnumerable<Point> positions);
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

		public Character CreateCharacter (Point position, Health health, Defense defense)
		{
			return new Character (NextCharacterID (), position, 100, ImmutableList<Skill>.Empty, health, defense);
		}

		public Character CreatePlayer (Point position, Health health, Defense defense)
		{
			return new Character (Character.PlayerID, position, 100, ImmutableList<Skill>.Empty, health, defense);
		}

		public SpawnerScript CreateSpawner (Point position)
		{
			return new SpawnerScript (NextScriptID (), 100, position, 5, 3);
		}

		public Skill CreateSkill (string name, Effect effect, TargettingInfo targetInfo, SkillResources resources, int power)
		{
			return new Skill (NextSkillID (), name, effect, targetInfo, resources, power);
		}

		public ReduceCooldownScript CreateCooldownScript (int ct, Character character, Skill skill)
		{
			return new ReduceCooldownScript (NextScriptID (), ct, character.ID, skill.ID);
		}

		public AreaDamageScript CreateDamageScript (int ct, int damage, ImmutableHashSet<Point> affectedPoints)
		{
			return new AreaDamageScript (NextScriptID (), ct, damage, affectedPoints);
		}

		public Character CreateStubPlayer (Point position)
		{
			return CreatePlayer (position, new Health (1, 1), new Defense (0));
		}

		public Character CreateStubEnemy (Point position)
		{
			return CreateCharacter (position, new Health (1, 1), new Defense (0));
		}

		public ImmutableList<Character> CreateStubEnemies (IEnumerable<Point> positions)
		{
			return positions.Select (x => CreateCharacter (x, new Health (1,1), new Defense (0))).ToImmutableList ();
		}

	}
}