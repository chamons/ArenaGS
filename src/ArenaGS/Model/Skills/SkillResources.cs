using System;
using ProtoBuf;

namespace ArenaGS.Model
{
	[ProtoContract]
	public sealed class SkillResources
	{
		[ProtoMember(1)]
		public int MaxAmmo { get; private set; }

		[ProtoMember(2)]
		public int CurrentAmmo { get; private set; }

		[ProtoMember(3)]
		public int Cooldown { get; private set; }

		[ProtoMember(4)]
		public int MaxCooldown { get; private set; }

		[ProtoMember(5)]
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

		public SkillResources (int maxAmmo = -1, int maxCooldown = -1, bool rechargedAmmo = false) : this(maxAmmo, maxAmmo, 0, maxCooldown, rechargedAmmo)
		{
		}

		public static SkillResources None => new SkillResources(-1, -1);
		public static SkillResources WithAmmo (int ammo) => new SkillResources(maxAmmo: ammo);
		public static SkillResources WithCooldown (int cooldown) => new SkillResources(maxCooldown: cooldown);
		public static SkillResources WithRechargingAmmo (int ammo, int cooldown) => new SkillResources(maxAmmo: ammo, maxCooldown: cooldown, rechargedAmmo: true);

		public SkillResources WithLessAmmo ()
		{
			if (!HasAmmoRemaining)
				throw new InvalidOperationException();

			return new SkillResources(this) { CurrentAmmo = CurrentAmmo - 1 };
		}

		public SkillResources WithIncrementedAmmo ()
		{
			if (!UsesAmmo)
				throw new InvalidOperationException();

			return new SkillResources(this) { CurrentAmmo = CurrentAmmo + 1 };
		}

		public SkillResources WithReloadedAmmo ()
		{
			if (!UsesAmmo)
				throw new InvalidOperationException();

			return new SkillResources(this) { CurrentAmmo = MaxAmmo };
		}

		public SkillResources WithCooldownSet ()
		{
			if (!UsesCooldown)
				throw new InvalidOperationException();

			return new SkillResources(this) { Cooldown = MaxCooldown };
		}

		public SkillResources WithCooldownReduced ()
		{
			if (!UnderCooldown)
				throw new InvalidOperationException();

			return new SkillResources(this) { Cooldown = Cooldown - 1 };
		}

		public override string ToString ()
		{
			return $"SkillResource {CurrentAmmo}/{MaxAmmo} {Cooldown}/{MaxCooldown}";
		}
	}
}
