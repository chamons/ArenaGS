using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ArenaGS.Platform
{
    public interface IFileStorage
    {
		string SaveLocation { get; }

		bool FileExists (string filename);
		void SaveFile (string filename, byte[] contents);
		byte[] LoadFile (string filename);
	}
}
