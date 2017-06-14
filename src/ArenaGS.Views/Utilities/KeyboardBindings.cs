using ArenaGS.Utilities;
using Optional;

namespace ArenaGS.Views.Utilities
{
	static class KeyboardBindings
	{
		internal static Option<Direction> IsDirectionKey (string character)
		{
			switch (character)
			{
				case "Up":
				case "NumPad8":
					return Direction.North.Some ();
				case "Down":
				case "NumPad2":
					return Direction.South.Some ();
				case "Left":
				case "NumPad4":
					return Direction.West.Some ();
				case "Right":
				case "NumPad6":
					return Direction.East.Some ();
				case "NumPad7":
					return Direction.Northwest.Some ();
				case "NumPad9":
					return Direction.Northeast.Some ();
				case "NumPad1":
					return Direction.Southwest.Some ();
				case "NumPad3":
					return Direction.Southeast.Some ();
				default:
					return Option.None<Direction> ();
			}
		}

		internal static bool IsReturn (string character)
		{
			switch (character)
			{
				case "\r":
				case "Return":
					return true;
				default:
					return false;
			}
		}
	}
}
