using System.Collections.Immutable;
using System.Linq;

using ArenaGS.Engine;
using ArenaGS.Utilities;
using ProtoBuf;

namespace ArenaGS.Model
{
	[ProtoContract]
	public sealed class Character : ITimedElement
	{
		[ProtoMember (1)]
		public int ID { get; private set; }

		[ProtoMember (2)]
		public string Name { get; private set; }

		[ProtoMember (3)]
		public Point Position { get; private set; }

		[ProtoMember (4)]
		public int CT { get; private set; }

		[ProtoMember (5)]
		public ImmutableList<Skill> Skills { get; private set; }

		[ProtoMember (6)]
		public Health Health { get; private set; }

		[ProtoMember (7)]
		public Defense Defense { get; private set; }

		public Character ()
		{
		}

		public Character (int id, string name, Point position, int ct, ImmutableList<Skill> skills, Health health, Defense defense)
		{
			ID = id;
			Name = name;
			Position = position;
			CT = ct;
			Skills = skills;
			Health = health;
			Defense = defense;
		}

		Character (Character original)
		{
			ID = original.ID;
			Name = original.Name;
			Position = original.Position;
			CT = original.CT;
			Skills = original.Skills;
			Health = original.Health;
			Defense = original.Defense;
		}

		public override string ToString () => $"{Name} {ID} - {Position} {CT} {Health}";

		internal Character WithID (int id)
		{
			return new Character (this) { ID = id };
		}

		internal Character WithPosition (Point position)
		{
			return new Character (this) { Position = position };
		}

		internal Character WithPosition (Point position, int ct)
		{
			return new Character (this) { Position = position, CT = ct };
		}

		internal Character WithReducedCT (int ct)
		{
			return WithCT (CT - ct);
		}

		internal Character WithAdditionalCT (int additionalCT)
		{
			return WithCT (CT + additionalCT);
		}

		internal Character WithCT (int ct)
		{
			return new Character (this) { CT = ct };
		}
		
		ITimedElement ITimedElement.WithAdditionalCT (int additionalCT)
		{
			return WithAdditionalCT (additionalCT);
		}

		internal Character WithSkills (ImmutableList<Skill> skills)
		{
			return new Character (this) { Skills = skills };
		}

		internal Character WithAdditionalSkill (Skill skill)
		{
			return new Character (this) { Skills = Skills.Add (skill) };
		}

		internal Character WithHealth (Health health)
		{
			return new Character (this) { Health = health };
		}

		internal Character WithCurrentHealth (int currentHealth)
		{
			return WithHealth (Health.WithCurrentHealth (currentHealth));
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

		public const int PlayerID = 42;
		public bool IsPlayer => ID == PlayerID;
	}
}