using ProtoBuf;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ArenaGS.Model
{
	[ProtoContract]
	public sealed class EffectResistance
	{
		[ProtoMember (1)]
		public ImmutableDictionary<string, int> DiminishingReturns { get; private set; }

		public EffectResistance ()
		{
			DiminishingReturns = ImmutableDictionary<string, int>.Empty;
		}

		EffectResistance (EffectResistance original)
		{
			DiminishingReturns = original.DiminishingReturns;
		}

		internal EffectResistance WithResistanceIncremented (string s)
		{
			ImmutableDictionary<string, int> dr = null;
			if (DiminishingReturns.ContainsKey (s))
				dr = DiminishingReturns.SetItem (s, DiminishingReturns [s] + 1);
			else
				dr = DiminishingReturns.SetItem (s, 1);

			return new EffectResistance (this) { DiminishingReturns = dr };
		}

		internal EffectResistance Clear ()
		{
			return new EffectResistance (this) { DiminishingReturns = DiminishingReturns.Clear () };
		}

		public int this [string key]
		{
			get
			{
				int v;
				if (DiminishingReturns.TryGetValue (key, out v))
					return v;
				return 0;
			}
		}
	}
}
