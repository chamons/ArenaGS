﻿using ArenaGS.Engine;
using ArenaGS.Utilities;

using ProtoBuf;

namespace ArenaGS.Model
{
	[ProtoContract]
	[ProtoInclude (500, typeof (SpawnerScript))]
	[ProtoInclude (500, typeof (ReduceCooldownScript))]
	public abstract class MapScript : ITimedElement
	{
		[ProtoMember (1)]
		public int ID { get; private set; }

		[ProtoMember (2)]
		public int CT { get; protected set; }

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

		public abstract MapScript WithCT (int ct);
		public abstract MapScript WithAdditionalCT (int additionalCT);
	}

	[ProtoContract]
	public sealed class SpawnerScript : MapScript
	{
		[ProtoMember (3)]
		public Point Position { get; private set; }

		[ProtoMember (4)]
		public int Cooldown { get; private set; }

		[ProtoMember (5)]
		public int TotalToSpawn { get; private set; }

		[ProtoMember (6)]
		public int TimeToNextSpawn { get; private set; }

		[ProtoMember (7)]
		public int SpawnCount { get; private set; }

		public SpawnerScript (int id, Point position, int ct, int spawnCount, int cooldown) : base (id, ct)
		{
			Position = position;
			Cooldown = cooldown;
			TotalToSpawn = spawnCount;
			TimeToNextSpawn = Cooldown;
			SpawnCount = 0;
		}

		SpawnerScript (SpawnerScript script) : base (script)
		{
			Position = script.Position;
			Cooldown = script.Cooldown;
			TotalToSpawn = script.TotalToSpawn;
			TimeToNextSpawn = script.TimeToNextSpawn;
			SpawnCount = script.SpawnCount;
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
		[ProtoMember (3)]
		public int CharacterID { get; private set; }

		[ProtoMember (4)]
		public int SkillID { get; private set; }

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
}
