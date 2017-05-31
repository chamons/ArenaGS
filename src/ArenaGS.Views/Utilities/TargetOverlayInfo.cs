using ArenaGS.Utilities;

namespace ArenaGS.Views.Utilities
{
	public class TargetOverlayInfo
	{
		public Point Position { get; }
		public int Area { get; }
		public bool Valid { get; }

		public TargetOverlayInfo (Point position, int area, bool valid)
		{
			Position = position;
			Area = area;
			Valid = valid;
		}
	}
}
