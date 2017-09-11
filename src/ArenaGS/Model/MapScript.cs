using System;
using ArenaGS.Engine;
using ArenaGS.Utilities;

using ProtoBuf;
using System.Collections.Immutable;

namespace ArenaGS.Model
{
	[ProtoContract]
	[ProtoInclude (500, typeof (SpawnerScript))]
	[ProtoInclude (600, typeof (ReduceCooldownScript))]
	[ProtoInclude (700, typeof (AreaDamageScript))]
	[ProtoInclude (800, typeof (TestScript))]
	public class MapScript : ITimedElement
	{
		[ProtoMember (10)]
		public int ID { get; private set; }

		[ProtoMember (11)]
		public int CT { get; protected set; }

		public MapScript ()
		{
		}

		public MapScript (int id, int ct)
		{
			ID = id;
			CT = ct;
		}

		protected MapScript (MapScript script)
		{
			ID = script.ID;
			CT = script.CT;
		}

		public virtual MapScript WithCT (int ct) { return this;  }
		public virtual MapScript WithAdditionalCT (int additionalCT) { return this; }

		ITimedElement ITimedElement.WithAdditionalCT (int additionalCT)
		{
			return WithAdditionalCT (additionalCT);
		}
	}

	[ProtoContract]
	public sealed class SpawnerScript : MapScript
	{
		[ProtoMember (20)]
		public Point Position { get; private set; }

		[ProtoMember (21)]
		public int Cooldown { get; private set; }

		[ProtoMember (22)]
		public int TotalToSpawn { get; private set; }

		[ProtoMember (23)]
		public int TimeToNextSpawn { get; private set; }

		[ProtoMember (24)]
		public int SpawnCount { get; private set; }

		[ProtoMember (25)]
		public string SpawnName { get; private set; }

		public SpawnerScript ()
		{
		}

		public SpawnerScript (int id, int ct, Point position, string spawnName, int spawnCount, int cooldown) : base (id, ct)
		{
			Position = position;
			Cooldown = cooldown;
			TotalToSpawn = spawnCount;
			TimeToNextSpawn = Cooldown;
			SpawnCount = 0;
			SpawnName = spawnName;
		}

		SpawnerScript (SpawnerScript script) : base (script)
		{
			Position = script.Position;
			Cooldown = script.Cooldown;
			TotalToSpawn = script.TotalToSpawn;
			TimeToNextSpawn = script.TimeToNextSpawn;
			SpawnCount = script.SpawnCount;
			SpawnName = script.SpawnName;
		}

		public override MapScript WithCT (int ct)
		{
			return new SpawnerScript (this) { CT = ct };
		}

		public MapScript WithTimeToNextSpawn (int timeToNextSpawn)
		{
			return new SpawnerScript (this) { TimeToNextSpawn = timeToNextSpawn };
		}

		public override MapScript WithAdditionalCT (int additionalCT)
		{
			return WithCT (CT + additionalCT);
		}

		public MapScript AfterSpawn ()
		{
			return new SpawnerScript (this) { TimeToNextSpawn = Cooldown, SpawnCount = SpawnCount + 1 };
		}

		public MapScript DecrementSpawnTimer ()
		{
			return WithTimeToNextSpawn (TimeToNextSpawn - 1);
		}
	}

	public sealed class ReduceCooldownScript : MapScript
	{
		[ProtoMember (30)]
		public int CharacterID { get; private set; }

		[ProtoMember (31)]
		public int SkillID { get; private set; }

		public ReduceCooldownScript ()
		{
		}

		public ReduceCooldownScript (int id, int ct, int characterID, int skillID) : base (id, ct)
		{
			CharacterID = characterID;
			SkillID = skillID;
		}

		ReduceCooldownScript (ReduceCooldownScript script) : base (script)
		{
			CharacterID = script.CharacterID;
			SkillID = script.SkillID;
		}

		public override MapScript WithCT (int ct)
		{
			return new ReduceCooldownScript (this) { CT = ct };
		}

		public override MapScript WithAdditionalCT (int additionalCT)
		{
			return WithCT (CT + additionalCT);
		}
	}

	// Don't stand in fire...
	public sealed class AreaDamageScript : MapScript
	{
		[ProtoMember (40)]
		public int Damage { get; private set; }

		[ProtoMember (41)]
		public ImmutableHashSet <Point> Area { get; private set; }

		public AreaDamageScript ()
		{
		}

		public AreaDamageScript (int id, int ct, int damage, ImmutableHashSet<Point> area) : base (id, ct)
		{
			Damage = damage;
			Area = area;
		}

		AreaDamageScript (AreaDamageScript script) : base (script)
		{
			Damage = script.Damage;
			Area = script.Area;
		}

		public override MapScript WithCT (int ct)
		{
			return new AreaDamageScript (this) { CT = ct };
		}

		public override MapScript WithAdditionalCT (int additionalCT)
		{
			return WithCT (CT + additionalCT);
		}
	}

	[ProtoContract]
	public class TestScript : MapScript
	{
		public TestScript () : base ()
		{
		}

		public TestScript (int id, int ct) : base (id, ct)
		{
		}

		protected TestScript (MapScript script) : base (script)
		{
		}

		public override MapScript WithCT (int ct) => new TestScript (this) { CT = ct };
		public override MapScript WithAdditionalCT (int additionalCT) => WithCT (CT + additionalCT);
	}
}
