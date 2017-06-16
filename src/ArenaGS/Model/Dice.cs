using ArenaGS.Engine;
using ProtoBuf;
using System;
using System.Globalization;
using System.Xml;

namespace ArenaGS.Model
{
	[ProtoContract]
	public class Dice
	{
		public static Dice Invalid = new Dice (-1, -1, -1);
		public static Dice Zero = new Dice (0, 0, 0);

		[ProtoMember (1)]
		public int Rolls { get; }

		[ProtoMember (2)]
		public int Faces { get; }

		[ProtoMember (3)]
		public int Constant { get; }

		public Dice (int rolls, int faces, int constant = 0)
		{
			Rolls = rolls;
			Faces = faces;
			Constant = constant;
		}

		public Dice Add (Dice other)
		{
			if (other.Equals (Zero))
				return this;

			if (this.Equals (Zero))
				return other;

			if (Faces != other.Faces)
				throw new InvalidOperationException ($"Can't add dice: {this} + {other}");

			return new Dice (Rolls + other.Rolls, Faces, Constant + other.Constant);
		}

		public override bool Equals (Object obj)
		{
			if (obj == null)
				return false;

			if (obj is Dice dice)
				return Equals (dice);

			return false;
		}

		public override int GetHashCode ()
		{
			return Rolls ^ Faces ^ Constant;
		}

		public bool Equals (Dice other) => Rolls == other.Rolls && Faces == other.Faces && Constant == other.Constant;

		public override string ToString () => $"{Rolls}d{Faces} + {Constant}";
	}

	public static class DiceExtensions
	{
		public static int Roll (this Dice dice, IRandomGenerator random)
		{
			int total = 0;
			for (int i = 0; i < dice.Rolls; i++)
				total += random.Roll (1, dice.Faces);
			return total + dice.Constant;
		}

		public static int RollMax (this Dice dice)
		{
			return (dice.Faces * dice.Rolls) + dice.Constant;
		}
	}
}
