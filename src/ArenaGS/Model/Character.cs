using ArenaGS.Utilities;

namespace ArenaGS.Model
{
	public class Character
	{
		public Point Position { get; private set; }

		public Character (Point position)
		{
			Position = position;
		}

		Character (Character original)
		{
			Position = original.Position;
		}

		internal Character WithNewPosition (Point position)
		{
			return new Character (this) { Position = position };
		}
	}
}
