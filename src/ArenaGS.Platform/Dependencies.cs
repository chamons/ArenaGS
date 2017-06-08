﻿using System;
using System.Collections.Generic;

namespace ArenaGS.Platform
{
	public static class Dependencies
	{
		static Dictionary<Type, object> Items = new Dictionary<Type, object> ();
		static Dictionary<Type, Type> Types = new Dictionary<Type, Type> ();

		public static void Register<T> (Type type) => Types [typeof (T)] = type;
		public static void RegisterInstance<T> (object value) => Items [typeof (T)] = value;

		public static T Get<T> ()
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

		public static void Unregister<T> ()
		{
			if (Items.ContainsKey (typeof (T)))
				Items.Remove (typeof (T));
			if (Types.ContainsKey (typeof (T)))
				Types.Remove (typeof (T));
		}

		public static void Clear ()
		{
			Items.Clear ();
			Types.Clear ();
		}
	}
}
