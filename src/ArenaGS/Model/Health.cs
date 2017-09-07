using ProtoBuf;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ArenaGS.Model
{
	[ProtoContract]
	public class Health
	{
		[ProtoMember (1)]
		public int Current { get; private set; }

		[ProtoMember (2)]
		public int Maximum { get; private set; }

		public Health ()
		{
		}

		public Health (int health)
		{
			Current = health;
			Maximum = health;
		}

		public Health (int current, int max)
		{
			Current = current;
			Maximum = max;
		}

		Health (Health original)
		{
			Current = original.Current;
			Maximum = original.Maximum;
		}

		public override string ToString () => $"{Current}/{Maximum}";

		internal Health WithCurrentHealth (int current)
		{
			return new Health (this) { Current = current };
		}
	}
}
