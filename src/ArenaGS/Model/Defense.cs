using ProtoBuf;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ArenaGS.Model
{
	[ProtoContract]
	public class Defense
	{
		[ProtoMember (1)]
		public int StandardDefense { get; private set; }

		public Defense ()
		{
		}

		public Defense (int value)
		{
			StandardDefense = value;
		}

		Defense (Defense original)
		{
			StandardDefense = original.StandardDefense;
		}
	}
}
