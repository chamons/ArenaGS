An overview of the design, from bottom to top assembly:

## Platform

Contains the minimum set that all other assemblies need to operate together.

- Logging
- Definition of file storage API
- Dependency finder used to find logging

## ArenaGS (Engine)

Contains all "model" objects and components to mutate them as actions are submitted.

- GameEngine is the primary facade used by the UI to request actions based on user input
    - Recieves commands via AcceptCommand and notifies UI for updates with StateChanged / AnimationRequested events
- QueryGameState is a secondary facade used to request additional read only information on the current state.
- GameState is a non-mutable snap shot of the current state of the game. It is passed to the UI on turn / animation updates for display
    - GameState classes are found in Engine/Model and contain no behavior or mutability.
- Classes in the Engine folder mutate the game state based on requested actions (requested by GameEngine)
    - Behaviors provide additional requests on the engine, such as enemies taking their turn when requested.
       - The GameState model classes have no behavior of their own
   - Generators create new content such as maps and enemies
   - Utilities provide services such as pathfinding and line calculations to any engine component
   
## Tests
   
Contains nunit tests for the logic within the engine class.

- Occasionally  stubs dependencies via Dependency dictionary to isolate cross component behaviors
- Only contains "model" tests. There are no UI tests, nor any planned right now.

## Views

Contains cross platform UI and all code to process user input

- GameController is the top level coordinator object instaned by Platform specific startup code
    - Passed in a IGameWindow interface with events to handle user input and a method to invalidate to request drawing
- A Scene object handles displaying and handling input for the current game state
    - Combat, shopping, credits, options all will be different scenes
    - The combat scene contains a number of "overlays" that handle displaying overlays and processing user input in different ways
        - Targetting a skill has different behavior when compared to looking around or moving
- Each scene owns a number of Views, which display parts of the world and hit test for mouse/touch interactions

## Platform Launchers

ArenaGS.Mac and ArenaGS.Windows are platform specific launchers which:

- Instance up a GameController and pass UI events to it (via IWindow interface)
- Provide an IFileStorage instance which describes where and how to write save/log files
- Last stop to handle any uncaught exceptions to display to the user.
