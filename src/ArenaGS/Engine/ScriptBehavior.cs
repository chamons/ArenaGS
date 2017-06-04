using System;
using System.Linq;
using ArenaGS.Engine;
using ArenaGS.Model;

namespace ArenaGS
{
	public interface IScriptBehavior
	{
		GameState Act (GameState state, MapScript script);
	}

	public class ScriptBehavior : IScriptBehavior
	{
		ITime Time;
		public ScriptBehavior ()
		{
			Time = Dependencies.Get<ITime> ();
		}

		public GameState Act (GameState state, MapScript script)
		{
			if (script is SpawnerScript spawnerScript)
			{
				if (spawnerScript.SpawnCount < spawnerScript.TotalToSpawn) 
				{
					if (spawnerScript.TimeToNextSpawn == 0) 
					{
						state = state.WithEnemies (state.Enemies.Add (Character.Create (script.Position)));
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
