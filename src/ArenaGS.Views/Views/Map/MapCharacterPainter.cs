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
			Images ["Golem"] = Resources.Get ("earth_elemental.png");

			Images ["Bow"] = Resources.Get ("bow_two.png");
			Images ["Shield"] = Resources.Get ("shield_kite1.png");
			Images ["Sword"] = Resources.Get ("sword_two.png");
			Images ["Axe"] = Resources.Get ("war_axe.png");
		}

		public IEnumerable<SKBitmap> GetImage (GameState state, int id)
		{
			Character c = state.AllCharacters.First (x => x.ID == id);
			if (c.IsPlayer)
				return new [] { Images ["Bow"], Images ["Player"] };

			switch (c.Name)
			{
				case "Wolf":
					return Images ["Wolf"].Yield ();
				case "Golem":
					return Images ["Golem"].Yield ();
				case "Skeleton":
					return new [] { Images ["Sword"], Images ["Shield"], Images ["Skeleton"] };
				case "Skeleton Archer":
					return new [] { Images ["Bow"], Images ["Skeleton"] };
				default:
					return Images ["Skeleton"].Yield ();
			}
		}

		public IEnumerable<SKBitmap> GetImage (GameState state, Character c)
		{
			return GetImage (state, c.ID);
		}
	}
}
