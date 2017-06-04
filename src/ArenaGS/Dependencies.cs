using System;
using System.Collections.Generic;

namespace ArenaGS
{
	internal static class Dependencies
	{
		static Dictionary<Type, object> Items = new Dictionary<Type, object> ();
		static Dictionary<Type, Type> Types = new Dictionary<Type, Type> ();

		internal static void Register<T> (Type type) => Types [typeof (T)] = type;
		internal static void RegisterInstance<T> (object value) => Items [typeof (T)] = value;

		internal static T Get<T> ()
		{
			object value;
			if (Items.TryGetValue (typeof (T), out value))
				return (T)value;

			Type type;
			if (Types.TryGetValue (typeof (T), out type))
			{
				value = Activator.CreateInstance (type);
				Items [typeof (T)] = value;
				return (T)value;
			}

			throw new InvalidOperationException ($"Dependency {typeof (T)} was not registered in the dependency dictionary.");
		}

		internal static void Unregister<T> ()
		{
			if (Items.ContainsKey (typeof (T)))
				Items.Remove (typeof (T));
			if (Types.ContainsKey (typeof (T)))
				Types.Remove (typeof (T));
		}
	}
}
