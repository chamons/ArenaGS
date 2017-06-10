using System.IO;
using ArenaGS.Platform;
using ProtoBuf;
using ArenaGS.Model;
using System.Collections.Immutable;
using ArenaGS.Engine.Generators;

namespace ArenaGS.Utilities
{
	internal static class Serialization
	{
		static IFileStorage Storage;

		static Serialization ()
		{
			Storage = Dependencies.Get<IFileStorage> ();
		}

		[ProtoContract]
		class SaveContainer
		{
			internal const int CurrentVersion = 1;

			[ProtoMember (1)]
			public int Version { get; private set; }
			[ProtoMember (2)]
			public GameState State { get; private set; }

			public SaveContainer ()
			{

			}

			public SaveContainer (GameState state)
			{
				Version = CurrentVersion;
				State = state;
			}
		}

		internal static bool SaveGameExists => Storage.FileExists (SaveFilePath);
		internal static string SaveFilePath => Storage.SaveLocation;

		internal static void Save (GameState state)
		{
			var container = new SaveContainer (state);

			using (MemoryStream ms = new MemoryStream ())
			{
				Serializer.Serialize (ms, container);
				Storage.SaveFile (SaveFilePath, ms.ToArray ());
			}
		}

		internal static GameState Load ()
		{
			byte [] compressedData = Storage.LoadFile (SaveFilePath);
			Storage.DeleteFile (SaveFilePath);
			using (MemoryStream os = new MemoryStream (compressedData))
			{
				SaveContainer container = Serializer.Deserialize<SaveContainer> (os);
				if (container.Version != SaveContainer.CurrentVersion)
					return null;

				Map savedStubMap = container.State.Map;
				var worldGenerator = Dependencies.Get<IWorldGenerator> ();
				Map map = worldGenerator.GetMapGenerator (savedStubMap.MapType).Regenerate (savedStubMap.GenerateHash);
				GameState state = container.State.WithMap (map);

				// Protobuffer serializes empty lists as null
				// For each list in state, we must instead give an empty list
				// Else NRE will occur on use
				if (state.Scripts == null)
					state = state.WithScripts (ImmutableList<MapScript>.Empty);
				if (state.Enemies == null)
					state = state.WithEnemies (ImmutableList<Character>.Empty);
				if (state.LogEntries == null)
					state = state.WithLog (ImmutableList<string>.Empty);

				return state;
			}
		}
	}
}
