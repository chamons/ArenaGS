using System.Linq;

using ArenaGS.Model;
using ArenaGS.Platform;
using ArenaGS.Utilities;

namespace ArenaGS.Engine.Behavior
{
	public interface IScriptBehavior
	{
		GameState Act (GameState state, MapScript script);
	}

	public class ScriptBehavior : IScriptBehavior
	{
		ITime Time;
		IPhysics Physics;
		ICombat Combat;
		IAnimationRequest Animation;

		IGenerator Generator;

		public ScriptBehavior ()
		{
			Time = Dependencies.Get<ITime> ();
			Physics = Dependencies.Get<IPhysics> ();
			Combat = Dependencies.Get<ICombat> ();
			Generator = Dependencies.Get<IGenerator>();
			Animation = Dependencies.Get<IAnimationRequest> ();
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
			else if (script is AreaDamageScript damageScript)
			{
				state = HandleDamageScript (state, damageScript);
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
					state = state.WithAddedEnemy (Generator.CreateCharacter (spawnerScript.SpawnName, spawnerScript.Position));
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
			Character character = state.AllCharacters.FirstOrDefault (x => x.ID == script.CharacterID);
			// If our target is no longer in existance, just remove ourself
			if (character == null)
				return state.WithScripts (state.Scripts.Remove (script));

			Skill skill = character.Skills.First (x => x.ID == script.SkillID);
			state = state.WithReplaceCharacter (character.WithReplaceSkill (skill.WithCooldownReduced ()));
			character = state.UpdateCharacterReference (character);
			skill = character.UpdateSkillReference (skill);

			if (!skill.UnderCooldown)
			{
				state = state.WithRemovedScript (script);
				if (skill.Resources.RechargedAmmoOnCooldown && skill.Resources.CurrentAmmo < skill.Resources.MaxAmmo)
				{
					state = state.WithReplaceCharacter (character.WithReplaceSkill (skill.WithIncrementedAmmo ()));
					character = state.UpdateCharacterReference (character);
					skill = character.UpdateSkillReference (skill);

					if (skill.Resources.CurrentAmmo < skill.Resources.MaxAmmo)
					{
						state = state.WithReplaceCharacter (character.WithReplaceSkill (skill.WithCooldownSet ()));
						skill = character.UpdateSkillReference (skill);
						character = state.UpdateCharacterReference (character);
						state = state.WithAddedScript (Generator.CreateCooldownScript (0, character, skill));
					}
				}
			}

			return state;
		}

		GameState HandleDamageScript (GameState state, AreaDamageScript script)
		{
			Animation.Request (state, new SpecificAreaExplosionAnimationInfo (script.Area));

			foreach (Character damagedCharacter in state.AllCharacters.Where (x => script.Area.Contains (x.Position)))
				state = Combat.Damage (state, damagedCharacter, script.Damage);

			state = state.WithRemovedScript (script);
			return state;
		}

	}
}
