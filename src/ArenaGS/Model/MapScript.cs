using System;
using ArenaGS.Engine;
using ArenaGS.Utilities;
using ProtoBuf;

namespace ArenaGS.Model
{
	[ProtoContract]
	[ProtoInclude (500, typeof (SpawnerScript))]
	public abstract class MapScript : ITimedElement
	{
		[ProtoMember (1)]
		public int ID { get; private set; }

		[ProtoMember (2)]
		public Point Position { get; private set; }

		[ProtoMember (3)]
		public int CT { get; protected set; }

		// HACKID
		static int NextID = 1000;

		protected static int GetNextID ()
		{
			int next = NextID;
			NextID++;
			return next;
		}

		public MapScript (Point position, int id, int ct)
		{
			Position = position;
			ID = id;
			CT = ct;
		}

		protected MapScript (MapScript script)
		{
			Position = script.Position;
			ID = script.ID;
			CT = script.CT;
		}

		public abstract MapScript WithCT (int ct);
		public abstract MapScript WithAdditionalCT (int additionalCT);
	}

	[ProtoContract]
	public sealed class SpawnerScript : MapScript
	{
		[ProtoMember (4)]
		public int Cooldown { get; private set; }

		[ProtoMember (5)]
		public int TotalToSpawn { get; private set; }
		 
		[ProtoMember (6)]
		public int TimeToNextSpawn { get; private set; }

		[ProtoMember (7)]
		public int SpawnCount { get; private set; }

		public SpawnerScript (Point position, int spawnCount, int cooldown) : base (position, GetNextID (), 0)
		{
			Cooldown = cooldown;
			TotalToSpawn = spawnCount;
			TimeToNextSpawn = Cooldown;
			SpawnCount = 0;
		}

		SpawnerScript (SpawnerScript script) : base (script)
		{
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
}
