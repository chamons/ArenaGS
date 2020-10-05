# Help I Need Somebody

The last [few](https://github.com/chamons/ArenaGS/pull/220) [weeks](https://github.com/chamons/ArenaGS/pull/226) have been spent implementing a really solid help system. 

Roguelikes have a history for being dense and inscrutable, and I hope to head that off with a full help system, and in the future a tutorial as well.

It turned out to be significantly more difficult that I expected:

- To layout any complex text, you need a layout system that can wrap text. I knew these would be non-trivial, but this turned out to be a bit of a bug-farm.
- The help system involved "links" that you can click, so the layout system needs to detect and divide certain chunks as links and keep those together (and underlined).
- Log entries that can span more than one line completely busted my log model.
- I originally planning on having multiple help tooltips can be pinned, like 'At the Gates' and 'Old World', but it was too complex and unnecessary so I had to redesign.
- I am terrible at UI art, and had to try a few things before getting something passable.

Let dig into that point about log model for a second. 

Until now, every log entry was one line, so if I wanted 7 lines on the screen, I could start at an index and walk 7 forward, stopping if I hit the end of our log. Hitting Page Up/Down just changed our starting index. Once we write a new log entry, we 'snap' to the end. Easy, clear just from the text, and it fits cleanly in 'clash', the game engine component that stands without UI. 'Arena', the game UI can bind to it rather easily.

Once each line can wrap an arbitrary number of times, how we do any of that without laying out the text? One can not know how many lines a block of text is without laying it out, and that requires a font. And fonts very much do not belong in the game engine, where we store the index.

After flailing around a few approaches I believe the problem is intractable without actually laying out the text. No approach I tried both showed the correct log entires in all cases and correctly 'felt' right with scrolling.

Now the current log position is a UI concern and held in the log view. The log model raises events when new items are added, and the UI if attached snaps as needed. A bit indirect, but it works.

Here's what it tooltips look like:

![Tooltip Help](https://chamons.github.io/ArenaGS/images/tooltip-help.jpg)

and here's the full 'modal' help:

![Large Help](https://chamons.github.io/ArenaGS/images/modal-help.jpg)
