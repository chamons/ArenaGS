using ArenaGS.Engine;
using ArenaGS.Utilities;
using ProtoBuf;
using System.Collections.Immutable;
using System.Linq;

namespace ArenaGS.Model
{
	[ProtoContract]
	public sealed class Character : ITimedElement
	{
		[ProtoMember (1)]
		public int ID { get; private set; }

		[ProtoMember (2)]
		public Point Position { get; private set; }

		[ProtoMember (3)]
		public int CT { get; private set; }

		[ProtoMember (4)]
		public ImmutableList<Skill> Skills { get; private set; }

		[ProtoMember (5)]
		public Health Health { get; private set; }

		[ProtoMember (6)]
		public Defense Defense { get; private set; }

		public Character ()
		{
		}

		public Character (int id, Point position, int ct, ImmutableList<Skill> skills, Health health, Defense defense)
		{
			ID = id;
			Position = position;
			CT = ct;
			Skills = skills;
			Health = health;
			Defense = defense;
		}

		Character (Character original)
		{
			ID = original.ID;
			Position = original.Position;
			CT = original.CT;
			Skills = original.Skills;
			Health = original.Health;
			Defense = original.Defense;
		}

		public override string ToString ()
		{
			string debugName = IsPlayer ? "Player" : $"Character : {ID}";
			return $"{debugName} - {Position} {CT} {Health}";
		}

		internal Character WithPosition (Point position)
		{
			return new Character (this) { Position = position };
		}

		internal Character WithPosition (Point position, int ct)
		{
			return new Character (this) { Position = position, CT = ct };
		}

		internal Character WithAdditionalCT (int additionalCT)
		{
			return WithCT (CT + additionalCT);
		}

		internal Character WithCT (int ct)
		{
			return new Character (this) { CT = ct };
		}

		internal Character WithSkills (ImmutableList<Skill> skills)
		{
			return new Character (this) { Skills = skills };
		}

		internal Character WithHealth (Health health)
		{
			return new Character (this) { Health = health };
		}

		internal Character WithDefense (Defense defense)
		{
			return new Character (this) { Defense = defense };
		}

		internal Character WithReplaceSkill (Skill newSkill)
		{
			Skill oldSkill = Skills.First (x => x.ID == newSkill.ID);
			return new Character (this) { Skills = Skills.Replace (oldSkill, newSkill) };
		}

		internal Skill UpdateSkillReference (Skill oldSkill)
		{
			return Skills.First (x => x.ID == oldSkill.ID);
		}

		internal const int PlayerID = 42;
		public bool IsPlayer => ID == PlayerID;
	}
}