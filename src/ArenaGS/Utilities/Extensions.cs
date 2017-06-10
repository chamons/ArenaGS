using System;

namespace ArenaGS.Utilities
{
	public static class Extensions
	{
		public static bool IsOdd (this int x) => x % 2 == 1;
		public static bool IsEven (this int x) => x % 2 == 0;
		public static int MakeOdd (this int x) => IsOdd (x) ? x : x - 1;
	}

	public static class RandomExtensions
	{
		public static bool CoinFlip (this Random rng) => rng.Next ().IsOdd ();
	}
}
