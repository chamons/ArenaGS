using System;
using ProtoBuf;

namespace ArenaGS.Model
{
	[ProtoContract]
	[ProtoInclude (500, typeof (DamageSkillEffectInfo))]
	[ProtoInclude (600, typeof (DelayedDamageSkillEffectInfo))]
	[ProtoInclude (700, typeof (MoveAndDamageSkillEffectInfo))]
	[ProtoInclude (800, typeof (HealEffectInfo))]
	public class SkillEffectInfo
	{
		[ProtoMember (10)]
		public int Power { get; protected set; }

		public virtual SkillEffectInfo WithPower (int power)
		{
			return new SkillEffectInfo () { Power = power };
		}

		public static SkillEffectInfo None { get; } = new SkillEffectInfo();
	}

	[ProtoContract]
	public class DamageSkillEffectInfo : SkillEffectInfo
	{
		[ProtoMember(20)]
		public bool Knockback { get; private set; }

		[ProtoMember(21)]
		public bool Stun { get; private set; }

		[ProtoMember(22)]
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

		public override SkillEffectInfo WithPower (int power)
		{
			return new DamageSkillEffectInfo (power, this.Knockback, this.Stun, this.Charge);
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

		public override SkillEffectInfo WithPower (int power)
		{
			return new DelayedDamageSkillEffectInfo (power);
		}
	}

	[ProtoContract]
	public class MoveAndDamageSkillEffectInfo : SkillEffectInfo
	{
		[ProtoMember(30)]
		public int Range { get; private set; }

		public MoveAndDamageSkillEffectInfo ()
		{
		}

		public MoveAndDamageSkillEffectInfo (int power, int range)
		{
			Power = power;
			Range = range;
		}

		public override SkillEffectInfo WithPower (int power)
		{
			return new MoveAndDamageSkillEffectInfo (power, this.Range);
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

		public override SkillEffectInfo WithPower (int power)
		{
			return new HealEffectInfo (power);
		}
	}
}
