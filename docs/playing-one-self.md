# Playing One Self

I was not planning on another update so soon, but an awesome set of occurences compels me to write again.

It occured to me today that it would not be difficult to wire up another AI script to play as the player.

This buys us a few things:

- I could use it to run thousands of games with various balance changes and get a feeling if the numbers look right
- If I could hook it up to the UI, I could in theory turn ArenaGS into a screen saver.

The changeset to do setup the UI to play a dumb script that always passed your turn turned out to be rather straightforward. It actually worked first try.

I was curious how fast it was when it wasn't drawing the UI, and the auto tests already run the game 'headless'. After fixing one path bug (rust puts unit tests in a different location relative to the resource files) and some refactoring so the test could ask if the player won/lost it was alive.

1000 tests in just under 4 seconds. Let's try 10,000.

Then this happened:

![Crash Log](https://chamons.github.io/ArenaGS/images/playing-oneself-1.png)

So the game got itself into a crash state, and it turned out to be a valid set of circumstances:

- A character take takes up multiple squares (2x2)
- Was on square 8,1
- And asked the physics engine if it could move to 8,0
- Physics said, sure, if when you are in that new spot there is nothing you'd be on top of
- The point tried to enumerate the new squares it'd be in
- And one of them would be 8, -1, but we store these as unsigned
- And since in debug rust throws instead of giving you garbage for underflow - Boom

The idea of throwing random inputs at a system is not new, it's called [Monkey Testing](https://en.wikipedia.org/wiki/Monkey_testing). 

I was honestly shocked that _within 5 seconds of running my test_ that it found a really good bug.

I shelved working on making the script do more than wonder around, writing the logic was a bit more involved that I expect.

Until I make time to do that though, running the game against itself every so often will still be useful to find bugs. 