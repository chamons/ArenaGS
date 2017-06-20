﻿using System;

using ArenaGS.Utilities;
using ProtoBuf;

namespace ArenaGS.Model
{
	public enum Effect
	{
		None,
		Damage,
		DelayedDamage,
		Movement,
		MoveAndDamageClosest,
		Heal
	}

	[ProtoContract]
	public sealed class Skill
	{
		[ProtoMember (1)]
		public int ID { get; private set; }

		[ProtoMember (2)]
		public string Name { get; private set; }

		[ProtoMember (3)]
		public Effect Effect { get; private set; }

		[ProtoMember (4)]
		public SkillEffectInfo EffectInfo { get; private set; }

		[ProtoMember (5)]
		public TargettingInfo TargetInfo { get; private set; }

		[ProtoMember (6)]
		public SkillResources Resources { get; private set; }		

		public Skill ()
		{
		}

		public Skill (int id, string name, Effect effect, SkillEffectInfo effectInfo, TargettingInfo targetInfo, SkillResources resources)
		{
			ID = id;
			Name = name;
			Effect = effect;
			EffectInfo = effectInfo;
			TargetInfo = targetInfo;
			Resources = resources;
		}

		Skill (Skill original)
		{
			ID = original.ID;
			Name = original.Name;
			Effect = original.Effect;
			EffectInfo = original.EffectInfo;
			TargetInfo = original.TargetInfo;
			Resources = original.Resources;
		}

		public Skill WithEffectInfo (SkillEffectInfo newEfectInfo)
		{
			return new Skill (this) { EffectInfo = newEfectInfo };
		}

		public bool UsesAmmo => Resources.UsesAmmo;
		public bool HasAmmoRemaining => Resources.HasAmmoRemaining;
		public bool UsesCooldown => Resources.UsesCooldown;
		public bool UnderCooldown => Resources.UnderCooldown;
		public bool RechargedAmmoOnCooldown => Resources.RechargedAmmoOnCooldown;

		public bool ReadyForUse
		{
			get
			{
				if (RechargedAmmoOnCooldown && HasAmmoRemaining)
					return true;
				if (UsesAmmo && !HasAmmoRemaining)
					return false;
				if (UsesCooldown && UnderCooldown)
					return false;
				return true;
			}
		}

		public Skill WithResources (SkillResources newResources)
		{
			return new Skill (this) { Resources = newResources };
		}

		public Skill WithLessAmmo ()
		{
			return WithResources (Resources.WithLessAmmo ());
		}

		public Skill WithIncrementedAmmo ()
		{
			return WithResources (Resources.WithIncrementedAmmo ());
		}

		public Skill WithReloadedAmmo ()
		{
			return WithResources (Resources.WithReloadedAmmo ());
		}

		public Skill WithCooldownSet ()
		{
			return WithResources (Resources.WithCooldownSet ());
		}

		public Skill WithCooldownReduced ()
		{
			return WithResources (Resources.WithCooldownReduced ());
		}
	}
}
