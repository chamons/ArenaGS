# Progression

Progression is a core facet of modern game design.  Simply put, progression is the experience of “moving forward” in a game over time. Your characters gets stronger or levels ups, gains new abilities, and the challenges increase over time to match your new abilities. This progression may carry over between game sessions, unlocking new contact or increasing raw character strength to make it easier in the future to progress farther towards the end goal. This can be contrasted with the the feeling of “starting all over again”, when you fail to accomplish some challenge and you begin in the same state again.

## Catagories

For games with content that repeats as you make multiple ‘runs’ hoping to complete some goal (roguelikes, Slay the Spire, FTL, etc), I divide progression into three categories:

- Player Skill Progression - Over time one hopes to improve to knowledge of a game and tactics needed to complete it. Carries between game sessions passively. Classic roguelikes completely rely on player skill progression.
- Character Progression - The improvement in character abilities and power within a single run. You may begin with a simple sword and armor and end with a dozen magical items and a screen of different abilities. However when run ends this is reset and you begin again and the known ‘start’ state.
- ‘Meta’ Progression - The progression within the game between runs via unlocks and sometimes achievements. This may involve unlocking new characters or backgrounds, expanding the pool of items or cards, and sometimes entire new game modes. This can sometimes also unlock graduated difficulty settings (ascensions) to allow players to dial the difficulty to their skill level. In some games (Rogue Legacy, Dead Cells) this can directly improve your starting strength with in others (Slay the Spire, FTL) it just increases the number of decision choices available in a given run.

## Design Goals

With that bit of philosophy out of the way, what do I hope to achieve in ArenaGS? The primary design goals include:

- Each run’s characters should have at least some unique aspect to keep interest. Always having the same exact abilities makes runs feel identical.
- Decisions like “I’m more familiar with X but Y may be more powerful” lead to trade offs, and those are the interesting decisions. Few decisions should be obvious or power gamer IQ tests.
- Provide some “core” abilities, so each run is less prone to RNG ruining your plans (I really need X but it didn’t show up in the last Y chances). You can plan a bit around some base abilities, even if they possibly less powerful.
- Make abilities unique with tradeoffs. Try to avoid X is just a stronger Y to upgrade to, unless X is a core ability and Y is a rare reward.
- Decisions are interested but limited in amount. ArenaGS hopes to be a “coffee break” length game for now (< 1 hour). Spending half of that planning a complex build is not desired.

## The Plan

Focusing today on the in-run character progression, the plan includes:

- Starting the character with a very limited toolbox, and no choice but starting weapon (once we have more than one).
- After each fight, provide an opportunity to acquire new abilities and customize the character before starting the next flight.
- No changes (equipment or otherwise) in fight. This significantly simplified the UX, we don’t have to worry about changing equipment/skills mid-fight, and can generate a starting game state based upon decisions up front per fight.
- After each fight provide a combination of fixed currency and random reward.
- The random reward is one of (2-3?) cards or a choice to skip it for a small amount of currency.
- There is some skill tree or grid where the currency can be spent to unlock ‘core’ cards. 
- The subset of cards available here represent time spent in between fights, fiddling with equipment tweaking, exploring and researching, or time spent aligning with factions of the world.
- Those cards are “slotted” into a set of spots. Those equipped affect your character in the next fight, the rest join your general inventory for use in future fights.
- Slots include “weapon part” (red), “armor mod” (blue), “accessory item” (green), and mastery (purple). Each weapon will also include one weapon-specific slot (yellow). For the gunslinger it’s ammo.
- In between each fight you can change equipped skills freely.
- Slotted cards can provide new active abilities, conditional passive (if X happens then Y), and improve character attributes

## A build example

To make up an example, here’s one potential gunslinger build:

Weapon:
- Stippled grip - Move and Shoot abilities consume 5 less exhaustion.
- Recoil spring - Adds short range “Triple shot” ability
- Longer barrel - Extend all ability range by 1, but reduce strength of point blank (distance 1) abilities by 2.

Ammo:
- Magnum Ammo - Unlock magnum skills

Armor:
- Automated loader - Reloading is a free action and cycles ammo kind only takes half as long
- Runic - Passively adds +2 armor to the first attack that is not dodged, then goes on 1000 tick cool down to recharge.

Accessory:
- Cloak of Shadows - Move distance two and leave a shadow behind. It fires weak shots that pierce armor (but not dodge). 2 charges per battle
- Troll’s Blood Potion - Regenerate health for 300 ticks. 3 sips per battle.

This potential build is a short (but not point blank) range brawler. The longer barrel helps offset the lower range of magnum ammo, and the abilities focus on mobility. The steam power automated loader means you never have to stop to reload and cloak gives you an out when you are outmaneuvered. Runes inscribed on the armor help absorb the occasional hit, and a regeneration potion round things out with some sustainability in long fights where you can’t dodge every hit.

Creating a system where builds like the can organically arise is the plan.

Next will be writing up issues that laid out the work piece by piece and getting started.  