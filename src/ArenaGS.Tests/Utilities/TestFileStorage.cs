using System;
using ArenaGS.Platform;
using NUnit.Framework;

namespace ArenaGS.Tests.Utilities
{
	class TestFileStorage : IFileStorage
	{
		public string SaveLocation => "IN_MEMORY";
		byte[] LastSave;

		public bool FileExists (string filename) => filename == SaveLocation && LastSave != null;

		public byte[] LoadFile (string filename)
		{
			return LastSave;
		}

		public void SaveFile (string filename, byte[] contents)
		{
			Assert.AreEqual (SaveLocation, filename);
			LastSave = contents;
		}

		public void DeleteFile (string filename)
		{
			Assert.AreEqual (SaveLocation, filename);
			LastSave = null;
		}
	}
}
