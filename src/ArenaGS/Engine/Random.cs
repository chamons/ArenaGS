using System;

namespace ArenaGS.Engine
{
	public interface IRandomGenerator
	{
		int Roll (int min, int max);
	}

	class RandomGenerator : IRandomGenerator
	{
		Random rng;

		public RandomGenerator ()
		{
			rng = new Random ();
		}

		public RandomGenerator (int seed)
		{
			rng = new Random (seed);
		}

		public int Roll (int min, int max)
		{
			return rng.Next (min, max);
		}
	}
}
