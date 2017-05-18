using ArenaGS.Utilities;
using ProtoBuf;

namespace ArenaGS.Model
{
	[ProtoContract]
	public sealed class Character
	{
		[ProtoMember (1)]
		public Point Position { get; private set; }

		public Character ()
		{			
		}

		public Character (Point position)
		{
			Position = position;
		}

		Character (Character original)
		{
			Position = original.Position;
		}

		internal Character WithPosition (Point position)
		{
			return new Character (this) { Position = position };
		}
	}
}
