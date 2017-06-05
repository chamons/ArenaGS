using ArenaGS.Utilities;
using ProtoBuf;
using System.Collections.Immutable;
using ArenaGS.Engine;

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

		public Character ()
		{
		}

		public Character (int id, Point position, int ct, ImmutableList<Skill> skills)
		{
			ID = id;
			Position = position;
			CT = ct;
			Skills = skills;
		}

		Character (Character original)
		{
			ID = original.ID;
			Position = original.Position;
			CT = original.CT;
			Skills = original.Skills;
		}

		public override string ToString ()
		{
			string debugName = IsPlayer ? "Player" : $"Character : {ID}";
			return $"{debugName} - {Position} {CT}";
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

		internal const int PlayerID = 42;
		public bool IsPlayer => ID == PlayerID;
	}
}