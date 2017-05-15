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

		internal Character WithNewPosition (Point position)
		{
			Character newCharacter = Clone ();
			newCharacter.Position = position;
			return newCharacter;
		}

		Character Clone ()
		{
			return new Character (this.Position);
		}
	}
}
