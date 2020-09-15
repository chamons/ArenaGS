# Hello World

Welcome to the first development update for Arena: Gunpowder and Sorcery. 

It's a game that's been in development for about 3 months at this point as a side passion project. It's around 10k lines of source code, which sounds more than it really since everything is currently defined in source code. 

Before I got into details, let's see what it looks like today:

This is the full game window:

![Full Screen](https://chamons.github.io/ArenaGS/images/hello-world-1.jpg)

and these are stippets of the other two fights:

![Golem](https://chamons.github.io/ArenaGS/images/hello-world-2.jpg)

![Summoner](https://chamons.github.io/ArenaGS/images/hello-world-3.jpg)

Arena: Gunpowder and Sorcery, or ArenaGS for sure is a roguelike game in development using Rust and SDL2. 

Today it runs on Windows and macOS. Linux likely works, just untested. I'm hoping to get it running on web assembly once I've passed the preview stage.

Gameplay involves battling one or more "boss" monsters in a turn based small arena environment, until once side walks out. Then another harder fight will ensue until your run is over. I hope to distill the most interesting parts of fights from Crawl Stone Soup or TOME here. 

In the future there will be both:
  - In run progression (items giving new abilities, changing existing abillites, and some raw power)
  - Between runs (meta) progression. Unlocking additions fights, items, and classes

A lot of this part comes lovingly from Slay the Spire and Monster Train.

Art and aesthetics wise this is a love letter to 16 RPGs from the SNES era. The art while awesome, is paid and thus not part of the open source project.

Today the content includes:

- Three boss fights ( 'Tutorial' Golem, Giant Bird, Elemenalist & Summons)
- One weapon class (Gunslinger)
- One arena (Beach)

The balance is still in progress, I'm not sure you can beat the Fire Elemental right now, for example.

I've got a checklist of "release stuff" to do before doing a "friends and family" preview. This like help/tutorial, nicer UX, and removing develop hotkeys.

Once that is complete I'll iterate on the feedback for awhile before working on content.

After that in-round progression will be the next building block, but that might be awhile.