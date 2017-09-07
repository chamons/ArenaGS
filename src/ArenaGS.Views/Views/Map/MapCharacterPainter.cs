using ArenaGS.Model;
using SkiaSharp;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ArenaGS.Views.Views
{
	class MapCharacterPainter
	{
		Dictionary<string, SKBitmap> Images = new Dictionary<string, SKBitmap> ();
		
		public MapCharacterPainter ()
		{
			Images ["Player"] = Resources.Get ("orc_knight.png");
			Images ["Skeleton"] = Resources.Get ("skeletal_warrior.png");
			Images ["Wolf"] = Resources.Get ("wolf.png");
		}

		public SKBitmap GetImage (GameState state, int id)
		{
			Character c = state.AllCharacters.First (x => x.ID == id);
			if (c.IsPlayer)
				return Images ["Player"];

			switch (c.Name)
			{
				case "Wolf":
					return Images ["Wolf"];
				default:
					return Images ["Skeleton"];
			}
		}

		public SKBitmap GetImage (GameState state, Character c)
		{
			return GetImage (state, c.ID);
		}
	}
}
