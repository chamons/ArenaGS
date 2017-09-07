using ArenaGS.Engine;
using ArenaGS.Model;
using ArenaGS.Utilities;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ArenaGS.Tests.Utilities
{
	static class TestExtensions
	{
		public static GameState WithTestEnemy (this GameState state, IGenerator generator, Point point)
		{
			return WithTestEnemies (state, generator, point.Yield ());
		}

		public static GameState WithTestEnemies (this GameState state, IGenerator generator, IEnumerable<Point> points)
		{
			return state.WithEnemies (TestEnemyHelper.CreateTestEnemies (generator, points));
		}
	}

	static class TestEnemyHelper
	{
		public static Character CreateTestEnemy (IGenerator generator, Point p)
		{
			return generator.CreateCharacter ("TestEnemy", p);
		}

		public static ImmutableList<Character> CreateTestEnemies (IGenerator generator, IEnumerable<Point> points)
		{
			return points.Select (x => generator.CreateCharacter ("TestEnemy", x)).ToImmutableList ();
		}
	}
}