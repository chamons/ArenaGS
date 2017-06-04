﻿using ArenaGS.Utilities;
using ProtoBuf;

namespace ArenaGS.Model
{
	public enum Effect
	{
		None,
		Damage
	}

	public enum TargettingStyle
	{
		None,
		Point
	}

	[ProtoContract]
	public sealed class TargettingInfo
	{
		[ProtoMember (1)]
		public TargettingStyle TargettingStyle { get; private set; }

		[ProtoMember (2)]
		public int Range { get; private set; }

		[ProtoMember (3)]
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

		public bool IsValidTarget (Point sourceLocation, Point target)
		{
			switch (TargettingStyle)
			{
				case TargettingStyle.Point:
					return target.NormalDistance (sourceLocation) <= Range;
				case TargettingStyle.None:
				default:
					return true;
			}			
		}
	}

	[ProtoContract]
	public sealed class Skill
	{
		[ProtoMember (1)]
		public string Name { get; private set; }

		[ProtoMember (2)]
		public Effect Effect { get; private set; }

		[ProtoMember (3)]
		public TargettingInfo TargetInfo { get; private set; }

		public Skill ()
		{
		}

		public Skill (string name, Effect effect, TargettingInfo targetInfo)
		{
			Name = name;
			Effect = effect;
			TargetInfo = targetInfo;
		}

		Skill (Skill original)
		{
			Name = original.Name;
			Effect = original.Effect;
			TargetInfo = original.TargetInfo;
		}
	}
}
