using System;
using System.Collections.Generic;

namespace ArenaGS
{
	internal static class Dependencies
	{
		static Dictionary<Type, object> Items = new Dictionary<Type, object> ();

		internal static void Register<T> (object value) => Items[typeof(T)] = value;
		internal static T Get<T> () => (T)Items[typeof (T)];
	}
}
