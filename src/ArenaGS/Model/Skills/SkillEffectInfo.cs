﻿using System;
using ProtoBuf;

namespace ArenaGS.Model
{
	[ProtoContract]
	[ProtoInclude (500, typeof (DamageSkillEffectInfo))]
	[ProtoInclude (500, typeof (DelayedDamageSkillEffectInfo))]
	[ProtoInclude (500, typeof (MoveAndDamageSkillEffectInfo))]
	[ProtoInclude (500, typeof (HealEffectInfo))]
	public class SkillEffectInfo
	{
		[ProtoMember (1)]
		public int Power { get; protected set; }

		public static SkillEffectInfo None { get; } = new SkillEffectInfo();
	}

	[ProtoContract]
	public class DamageSkillEffectInfo : SkillEffectInfo
	{
		[ProtoMember(2)]
		public bool Knockback { get; private set; }

		[ProtoMember(3)]
		public bool Stun { get; private set; }

		[ProtoMember(3)]
		public bool Charge { get; private set; }

		public DamageSkillEffectInfo ()
		{
		}

		public DamageSkillEffectInfo (int power = 0, bool knockback = false, bool stun = false, bool charge = false)
		{
			Power = power;
			Knockback = knockback;
			Stun = stun;
			Charge = charge;
		}
	}

	[ProtoContract]
	public class DelayedDamageSkillEffectInfo : SkillEffectInfo
	{
		public DelayedDamageSkillEffectInfo ()
		{
		}

		public DelayedDamageSkillEffectInfo (int power)
		{
			Power = power;
		}
	}

	[ProtoContract]
	public class MoveAndDamageSkillEffectInfo : SkillEffectInfo
	{
		[ProtoMember(2)]
		public int Range { get; private set; }

		public MoveAndDamageSkillEffectInfo ()
		{
		}

		public MoveAndDamageSkillEffectInfo (int power, int range)
		{
			Power = power;
			Range = range;
		}
	}

	[ProtoContract]
	public class HealEffectInfo : SkillEffectInfo
	{
		public HealEffectInfo ()
		{
		}

		public HealEffectInfo (int power)
		{
			Power = power;
		}
	}
}
