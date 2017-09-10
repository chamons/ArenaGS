using System;
using System.Linq;
using System.Collections.Generic;
using System.Collections.Immutable;
using ArenaGS.Model;
using ArenaGS.Utilities;
using ArenaGS.Platform;

namespace ArenaGS.Engine
{
	public interface IGenerator
	{
		Character CreateCharacter (string name, Point position);
		Character CreatePlayer (Point position);
		Character CreateTestPlayer (Point position);

		SpawnerScript CreateSpawner (Point position, string spawnName);
		ReduceCooldownScript CreateCooldownScript (int ct, Character character, Skill skill);
		AreaDamageScript CreateDamageScript (int ct, int damage, ImmutableHashSet<Point> affectedPoints);

		Skill CreateSkill (string name, Effect effect, SkillEffectInfo effectInfo, TargettingInfo targetInfo, SkillResources resources);
		Skill CreateSkill (string name);
		Skill CreateSkill (string name, int power);
		Skill CreateSkill (string name, int power, string newName);
		Skill CreateSkill (string name, string newName);
	}

	public class Generator : IGenerator
	{
		const int CharacterOffset = 100;
		const int ScriptOffset = 2000;
		const int SkillOffset = 30000;

		SkillLibrary SkillLibrary;
		CharacterLibrary CharacterLibrary;

		public Generator ()
		{
			SkillLibrary = new SkillLibrary (this);
			CharacterLibrary = new CharacterLibrary (this);
		}

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

		public Character CreateCharacter (string name, Point position)
		{
			return CharacterLibrary.CreateCharacter (name).WithID (NextCharacterID ()).WithPosition (position);
		}

		public Character CreateTestPlayer (Point position)
		{
			return CharacterLibrary.CreateCharacter ("TestPlayer").WithPosition (position);
		}

		public Character CreatePlayer (Point position)
		{
			return CharacterLibrary.CreateCharacter ("Player").WithPosition (position);
		}

		internal Character CreateRawPlayer (Point position, Health health, Defense defense)
		{
			return new Character (Character.PlayerID, "Player", position, 100, ImmutableList<Skill>.Empty, health, defense);
		}

		internal Character CreateRawCharacter (string name, Point position, Health health, Defense defense)
		{
			return new Character (NextCharacterID (), name, position, 100, ImmutableList<Skill>.Empty, health, defense);
		}

		public SpawnerScript CreateSpawner (Point position, string spawnName)
		{
			return new SpawnerScript (NextScriptID (), 100, position, spawnName, 5, 3);
		}

		public Skill CreateSkill (string name)
		{
			return SkillLibrary.CreateSkill (name).WithID (NextSkillID ());
		}

		public Skill CreateSkill (string name, int power)
		{
			Skill ts = SkillLibrary.CreateSkill (name);
			return new Skill (NextSkillID (), ts.Name, ts.Effect, ts.EffectInfo.WithPower (power), ts.TargetInfo, ts.Resources);
		}

		public Skill CreateSkill (string name, string newName)
		{
			Skill ts = SkillLibrary.CreateSkill (name);
			return new Skill (NextSkillID (), newName, ts.Effect, ts.EffectInfo, ts.TargetInfo, ts.Resources);
		}

		public Skill CreateSkill (string name, int power, string newName)
		{
			Skill ts = SkillLibrary.CreateSkill (name);
			return new Skill (NextSkillID (), newName, ts.Effect, ts.EffectInfo.WithPower (power), ts.TargetInfo, ts.Resources);
		}

		public Skill CreateSkill (string name, Effect effect, SkillEffectInfo effectInfo, TargettingInfo targetInfo, SkillResources resources)
		{
			return new Skill (NextSkillID (), name, effect, effectInfo, targetInfo, resources);
		}

		public ReduceCooldownScript CreateCooldownScript (int ct, Character character, Skill skill)
		{
			return new ReduceCooldownScript (NextScriptID (), ct, character.ID, skill.ID);
		}

		public AreaDamageScript CreateDamageScript (int ct, int damage, ImmutableHashSet<Point> affectedPoints)
		{
			return new AreaDamageScript (NextScriptID (), ct, damage, affectedPoints);
		}
	}
}