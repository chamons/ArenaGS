using System.Collections.Generic;

public static class EnumerableExtensions
{
	public static IEnumerable<T> Yield<T> (this T item)
	{
		yield return item;
	}
}