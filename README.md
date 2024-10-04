### README

![Halo Object Viewer](https://github.com/lbfs/objectviewer/blob/main/screenshot.jpg?raw=true)

This tool was designed to research Halo 1 AUP. It is designed to work with Halo 1 Xbox on the XEMU emulator.

### We discovered the trick!

After a bunch of testing, the trick was finally able to be pulled out without any modifications for the game and on a real original xbox! Here is a video of me turning into the monitor!

https://www.twitch.tv/matabertus/clip/FreezingGentleBobaDatBoi-izmgmUE_1HijDWMn


### How does it work?

- In Halo 1, there are certain spots where when the game attempts to respawn a dead player, there will be no available positions around the currently alive player to place the second player. In this state, there is no UI feedback that the game is waiting to respawn the player, the camera will simply be focused on the currently alive player. This state is known as a pinch.
- When in the pinch state, the game has created an invisible player at the starting location in the map. If the pinch was done using a load zone by standing at the absolute border of the load zone, the invisible player will be inside of that load zone, however his activation would be skipped as this would trigger a BSP switch. This invisible player is what the respawn system intends to use as the character you will end up controlling once you are in a valid position in the map to allow the respawn.
- Normally, this invisible player cannot be seen or interacted with by the currently alive player. However, a vehicle or popcorn flood are some of the few things in the game that are able to kill this invisible player. You can tell this happens as the shield break effect will still show. After killing the invisible player, the garbage collector is now allowed to remove this invisible player from the current list of objects in the game.  
- However, the garbage collector only activates in certain circumstances, when the number of objects in the map exceeds a certain threshold or when certain scripts are run. For example, in "The Silent Cartographer," it activates when approaching the ramp before boarding the Pelican at the end of the level. In "The Library," it may trigger before the first elevator, depending on how many flood were killed and the number of objects were created. 
- If you have pinched your player and eliminate the invisible player after entering the waiting-to-respawn state, traversing one of these garbage collection spots will cause the invisible player to be deloaded. This action opens an entry in the object table while preserving the reference to that object in the respawn system. If you do not enter the waiting-to-respawn state and the invisible player dies, the game will simply create a new invisible player. 
- Once this entry in the object table is free, you can then place another object into this slot on the object table. Upon exiting the waiting-to-respawn status, the game will assign you to the object in that slot, provided that object is a unit and that unit is alive, creating a successful arbitrary unit possession. All characters and vehicles in the game are derived from unit. If this object is not an object that derives from unit, or the slot is empty when leaving waiting-to-respawn, the game will crash. 

### How do you stay in a waiting-to-respawn state long enough?
- One time consuming method that was discovered during researching this glitch is to be pinched or waiting-to-respawn when too many enemies are nearby for over 20 minutes. The game has an internal timer that will overflow. If you start walking around after waiting this amount of time, the game will not attempt to respawn your character for an additional 17 minutes. This can give you flexibility to pull off more complicated object table manipulations.
- The primary and fast method would be to use a plasma pistol or plasma rifle. You need to fire these weapons every second at the farthest point in your vision, this will keep the status in teammate in combat. Additionally, you can use a grenade to initiate the waiting-to-respawn status, as moving without doing so will likely cause a respawn, this will most likely be the first step you take when leaving most pinch spots. Similarly, passing through a load zone while in the waiting-to-respawn state will result in an immediate respawn, unless you are jumping through the load zone, which will cause the game to believe you are moving too fast. 

### How is this different from the Halo 2 AUP Glitch?

- Halo 2 requires that the entire object datum matches between the respawn system and the object table. The datum is a combination of an index, the position in the table, and an ID on the object that is used to validate if the object is the same. Halo 1 also stores the entire datum, but due to a bug with the respawn system, only the index is checked. If you match the ID by overflowing the ID counter through shooting 32K bullets, the game will additionally allow teleports and the view-model will be more correct. The ID must be matched in the Master Chief Collection in order to perform arbitrary unit possession. 
- Halo 2 has an additional glitch where the respawn system state is preserved across level resets, this is not the case in Halo 1, therefore making the quick method of setting up AUP in Halo 1 impossible. 
- Halo 2 allows you to delay respawn by using melee, you cannot do this in Halo 1.
