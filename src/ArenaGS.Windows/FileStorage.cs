using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ArenaGS.Platform;

namespace ArenaGS.Windows
{
	class FileStorage : IFileStorage
	{
		public string SaveLocation
		{
			get
			{
				string savedGamePath = System.Environment.ExpandEnvironmentVariables ("%USERPROFILE%\\Saved Games");
				return Path.Combine (savedGamePath, "Arena Gunpowder and Sorcery", "ArenaGS.sav");
			}
		}

		public byte [] LoadFile (string filename)
		{
			return File.ReadAllBytes (filename);
		}

		public void SaveFile (string filename, byte [] contents)
		{
			Directory.CreateDirectory (Path.GetDirectoryName (filename));
			File.WriteAllBytes (filename, contents);
		}

		public bool FileExists (string filename)
		{
			return File.Exists (filename);
		}

		public void DeleteFile (string filename)
		{
			File.Delete (filename);
		}
	}
}
