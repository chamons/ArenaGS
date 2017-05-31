﻿using ArenaGS.Utilities;
using ProtoBuf;
using System.Collections.Immutable;

namespace ArenaGS.Model
{
	[ProtoContract]
	public sealed class Character
	{
		internal static int PlayerID = 42;

		static int NextID = 100;
		static int GetNextID ()
		{
			int next = NextID;
			NextID++;
			return next;
		}

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

		internal static Character Create (Point position)
		{
			return new Character (GetNextID (), position, 100, ImmutableList<Skill>.Empty);
		}

		internal static Character CreatePlayer (Point position)
		{
			return new Character (PlayerID, position, 100, new Skill [] {
				new Skill ("Fireball", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 8)),
				new Skill ("Grenade", Effect.Damage, new TargettingInfo (TargettingStyle.Point, 4, 3))
			}.ToImmutableList ());
		}
	}
}