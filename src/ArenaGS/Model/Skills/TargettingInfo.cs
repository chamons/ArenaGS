using System;
using ProtoBuf;

namespace ArenaGS.Model
{
	public enum TargettingStyle
	{
		None,
		Point,
		Line,
		Cone
	}

	[ProtoContract]
	public sealed class TargettingInfo
	{
		[ProtoMember(1)]
		public TargettingStyle TargettingStyle { get; private set; }

		[ProtoMember(2)]
		public int Range { get; private set; }

		[ProtoMember(3)]
		public int Area { get; private set; }

		public TargettingInfo ()
		{
		}

		public TargettingInfo (TargettingStyle style, int range = 0, int area = 0)
		{
			TargettingStyle = style;
			Range = range;
			Area = area;
		}
	}
}
