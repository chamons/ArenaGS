using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using SkiaSharp;

namespace ArenaGS.Views
{
	static class Resources
	{
		static Dictionary<string, SKBitmap> LoadedResources = new Dictionary<string, SKBitmap> ();

		internal static void LoadResouces ()
		{
			var assembly = typeof (Resources).GetTypeInfo ().Assembly;

			foreach (var name in assembly.GetManifestResourceNames ().Where (x => x.EndsWith (".png", System.StringComparison.Ordinal)))
				LoadedResources.Add (FileNameFromResourceName (name), SKBitmap.Decode (assembly.GetManifestResourceStream (name)));
		}

		internal static SKBitmap Get (string name)
		{
			return LoadedResources[name];
		}

		static string FileNameFromResourceName (string resourceName)
		{
			var bits = resourceName.Split (new char[] { '.' });
			return bits[bits.Length - 2] + "." + bits[bits.Length - 1];
		}
	}
}
