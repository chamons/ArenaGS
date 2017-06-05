using System.Linq;

using ArenaGS.Model;

namespace ArenaGS.Engine.Behavior
{
	public interface IScriptBehavior
	{
		GameState Act (GameState state, MapScript script);
	}

	public class ScriptBehavior : IScriptBehavior
	{
		ITime Time;
		IGenerator Generator;

		public ScriptBehavior ()
		{
			Time = Dependencies.Get<ITime> ();
			Generator = Dependencies.Get<IGenerator>();
		}

		public GameState Act (GameState state, MapScript script)
		{
			if (script is SpawnerScript spawnerScript)
			{
				if (spawnerScript.SpawnCount < spawnerScript.TotalToSpawn) 
				{
					if (spawnerScript.TimeToNextSpawn == 0) 
					{
						state = Generator.CreateEnemy (state, script.Position);
						state = state.WithReplaceScript (spawnerScript.AfterSpawn ());
					}
					else
						state = state.WithReplaceScript (spawnerScript.DecrementSpawnTimer ());
				}
			}
			script = state.Scripts.First (x => x.ID == script.ID);
			state = state.WithReplaceScript (script.WithCT (Time.ChargeTime (script, TimeConstants.CTPerBasicAction)));
			return state;
		}
	}
}
