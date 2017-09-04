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

		public static TargettingInfo None ()
		{
			return new TargettingInfo (TargettingStyle.None, 0, 0);
		}

		public static TargettingInfo Point (int range = 0, int area = 0)
		{
			return new TargettingInfo (TargettingStyle.Point, range, area);
		}

		public static TargettingInfo Line (int range = 0)
		{
			return new TargettingInfo (TargettingStyle.Line, range, 0);
		}

		public static TargettingInfo Cone (int range = 0, int area = 0)
		{
			return new TargettingInfo (TargettingStyle.Cone, range, area);
		}

		TargettingInfo (TargettingStyle style, int range, int area)
		{
			TargettingStyle = style;
			Range = range;
			Area = area;
		}
	}
}
