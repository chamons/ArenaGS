using System;

using ArenaGS.Utilities;
using ProtoBuf;

namespace ArenaGS.Model
{
	public enum Effect
	{
		None,
		Damage,
		DelayedDamage
	}

	public enum TargettingStyle
	{
		None,
		Point,
		Cone
	}

	[ProtoContract]
	public sealed class TargettingInfo
	{
		[ProtoMember (1)]
		public TargettingStyle TargettingStyle { get; private set; }

		[ProtoMember (2)]
		public int Range { get; private set; }

		[ProtoMember (3)]
		public int Area { get; private set; }

		public TargettingInfo ()
		{
		}

		public TargettingInfo (TargettingStyle style, int range = 0, int area = 0)
		{
			TargettingStyle = style;
			Range = range;
			Area = area;
		}
	}

	[ProtoContract]
	public sealed class SkillResources
	{
		[ProtoMember (1)]
		public int MaxAmmo { get; private set; }

		[ProtoMember (2)]
		public int CurrentAmmo { get; private set; }

		[ProtoMember (3)]
		public int Cooldown { get; private set; }

		[ProtoMember (4)]
		public int MaxCooldown { get; private set; }

		[ProtoMember (5)]
		public bool RechargedAmmoOnCooldown { get; private set; }

		public bool UsesAmmo => MaxAmmo != -1;
		public bool HasAmmoRemaining => CurrentAmmo > 0;
		public bool UsesCooldown => MaxCooldown != -1;
		public bool UnderCooldown => Cooldown > 0;

		public SkillResources ()
		{
		}

		public SkillResources (int currentAmmo, int maxAmmo, int cooldown, int maxCooldown, bool rechargedAmmoOnCooldown)
		{
			CurrentAmmo = currentAmmo;
			MaxAmmo = maxAmmo;
			Cooldown = cooldown;
			MaxCooldown = maxCooldown;
			RechargedAmmoOnCooldown = rechargedAmmoOnCooldown;
		}

		public SkillResources (SkillResources original)
		{
			CurrentAmmo = original.CurrentAmmo;
			MaxAmmo = original.MaxAmmo;
			Cooldown = original.Cooldown;
			MaxCooldown = original.MaxCooldown;
			RechargedAmmoOnCooldown = original.RechargedAmmoOnCooldown;
		}

		public SkillResources (int maxAmmo = -1, int maxCooldown = -1, bool rechargedAmmo = false) : this (maxAmmo, maxAmmo, 0, maxCooldown, rechargedAmmo)
		{
		}

		public static SkillResources None => new SkillResources (-1, -1);
		public static SkillResources WithAmmo (int ammo) => new SkillResources (maxAmmo: ammo);
		public static SkillResources WithCooldown (int cooldown) => new SkillResources (maxCooldown: cooldown);
		public static SkillResources WithRechargingAmmo (int ammo, int cooldown) => new SkillResources (maxAmmo: ammo, maxCooldown: cooldown, rechargedAmmo: true);

		public SkillResources WithLessAmmo ()
		{
			if (!HasAmmoRemaining)
				throw new InvalidOperationException ();

			return new SkillResources (this) { CurrentAmmo = CurrentAmmo - 1 };
		}

		public SkillResources WithIncrementedAmmo ()
		{
			if (!UsesAmmo)
				throw new InvalidOperationException ();

			return new SkillResources (this) { CurrentAmmo = MaxAmmo };
		}

		public SkillResources WithReloadedAmmo ()
		{
			if (!UsesAmmo)
				throw new InvalidOperationException ();

			return new SkillResources (this) { CurrentAmmo = MaxAmmo };
		}

		public SkillResources WithCooldownSet ()
		{
			if (!UsesCooldown)
				throw new InvalidOperationException ();

			return new SkillResources (this) { Cooldown = MaxCooldown };
		}

		public SkillResources WithCooldownReduced ()
		{
			if (!UnderCooldown)
				throw new InvalidOperationException ();

			return new SkillResources (this) { Cooldown = Cooldown - 1 };
		}

		public override string ToString ()
		{
			return $"SkillResource {CurrentAmmo}/{MaxAmmo} {Cooldown}/{MaxCooldown}";
		}
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
		public TargettingInfo TargetInfo { get; private set; }

		[ProtoMember (5)]
		public SkillResources Resources { get; private set; }

		public Skill ()
		{
		}

		public Skill (int id, string name, Effect effect, TargettingInfo targetInfo, SkillResources resources)
		{
			ID = id;
			Name = name;
			Effect = effect;
			TargetInfo = targetInfo;
			Resources = resources;
		}

		Skill (Skill original)
		{
			ID = original.ID;
			Name = original.Name;
			Effect = original.Effect;
			TargetInfo = original.TargetInfo;
			Resources = original.Resources;
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
