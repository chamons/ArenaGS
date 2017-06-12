using System.Linq;

using ArenaGS.Model;
using ArenaGS.Platform;

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
				state = HandleSpawnerScript (state, spawnerScript);
				script = state.UpdateScriptReference (script);
			}
			else if (script is ReduceCooldownScript reduceCDScript)
			{
				state = HandleReduceCDScript (state, reduceCDScript);
				script = state.UpdateScriptReferenceIfExists (script);
			}
			
			if (script != null)
				state = state.WithReplaceScript (script.WithCT (Time.ChargeTime (script, TimeConstants.CTPerBasicAction)));
			return state;
		}

		GameState HandleSpawnerScript (GameState state, SpawnerScript spawnerScript)
		{
			if (spawnerScript.SpawnCount < spawnerScript.TotalToSpawn)
			{
				if (spawnerScript.TimeToNextSpawn == 0)
				{
					state = Generator.CreateEnemy (state, spawnerScript.Position);
					state = state.WithReplaceScript (spawnerScript.AfterSpawn ());
				}
				else
				{
					state = state.WithReplaceScript (spawnerScript.DecrementSpawnTimer ());
				}
			}

			return state;
		}

		GameState HandleReduceCDScript (GameState state, ReduceCooldownScript script)
		{
			Character character = state.AllCharacters.First (x => x.ID == script.CharacterID);
			Skill skill = character.Skills.First (x => x.ID == script.SkillID);
			skill = skill.WithCooldownReduced ();

			if (!skill.UnderCooldown)
			{
				state = state.WithScripts (state.Scripts.Remove (script));
				if (skill.UsesAmmo && skill.Resources.RechargedAmmoOnCooldown && skill.Resources.CurrentAmmo < skill.Resources.MaxAmmo)
				{
					state = state.WithReplaceCharacter (character.WithReplaceSkill (skill.WithIncrementedAmmo ()));
					character = state.UpdateCharacterReference (character);
					skill = character.UpdateSkillReference (skill);
				}
			}

			return state.WithReplaceCharacter (character.WithReplaceSkill (skill));
		}
	}
}
