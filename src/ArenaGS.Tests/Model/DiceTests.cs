using ArenaGS.Model;
using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ArenaGS.Tests.Model
{
	[TestFixture]
	public class DiceTests
	{
		[Test]
		public void DiceSimpleAddition_GivesExpectedValue ()
		{
			Dice one = new Dice (1, 1);
			Assert.AreEqual (Dice.Zero, Dice.Zero.Add (Dice.Zero));
			Assert.AreEqual (one, Dice.Zero.Add (one));
			Assert.AreEqual (new Dice (5, 3), (new Dice (2, 3)).Add (new Dice (3, 3)));
		}

		[Test]
		public void DiceInvalidAddition_Throws ()
		{
			Dice first = new Dice (2, 3);
			Dice second = new Dice (3, 4);
			Assert.Throws<InvalidOperationException> (() =>
			{
				Dice value = first.Add (second);
			});
		}

		[Test]
		public void DiceRoll_SmokeTest ()
		{
			Random rng = new Random (42);
			Dice dice = new Dice (3, 4, 2);
			int value = dice.Roll (rng);
			Assert.IsTrue (value >= 5 && value <= 14);
		}

		[Test]
		public void DiceRollMax_GivesCorrectValue ()
		{
			Dice dice = new Dice (3, 4, 2);
			Assert.AreEqual (14, dice.RollMax ());
		}
	}
}
