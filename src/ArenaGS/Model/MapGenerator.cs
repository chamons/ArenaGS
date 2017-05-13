namespace ArenaGS.Model
{
	internal interface IMapGenerator
	{
		Map Generate ();
	}
	
	internal class SimpleMapGenerator : IMapGenerator
	{
		public Map Generate ()
		{
			return new Map (40, 40);
		}
	}
}
