
![15puzzle_showcase](https://github.com/user-attachments/assets/ef2ef801-f15d-4555-8eca-33d48e611117)

A link to the game on itch: https://rabbival.itch.io/n-puzzle-with-walls

#### How to play
Try and get the numbers in the correct order, starting on the top left corner by rows.
Use WASD/Arrows to move the empty spaces around.
Alternatively, click the tile you'd like to move into the empty space with the mouse.

Pressing R would reroll numbered and empty arrangement on the board, while adding Shift would reroll walls too.
Use space to reopen the generation menu, Shift+ESC to close the app.
Pressing ESC when a pop-up message appears would close it, Enter would confirm it when the confirm button is visible.
Arrow keys may also be used for navigating the Loader screen.

Enjoy!


#### Some cool things I did here:
* Each grid can spawn attached data structures, such as "travellers" and a random tree graph based on them
    * The grid tree has a smart iterator so that it's exposed as a regular iterator but actually takes some smart steps in randomly choosing the next leaf
* While connectivity is ensured by using the tree subgraph (and full O(n) runs when it fails), I also ensured that:
    * Each numbered cell would have at least two neighbors (for the sake of more interesting shuffles, though it can also deal with cells that have one neighbor)
    * Each numbered cell is a part of a cycle (for the sake of movability, so that the number can be moved out of its original cell)
* Smart despawn and respawn - I tag all the numbers that get to stay in the next board and only remove the rest
    * This means that going down from a very large board to another large one is very efficient without damaging all other transitions
* Responsive and informative UI system- Errors are printed to the screen when managing saved boards and board generation. Only usable buttons show at any given time.


#### Acknowledgements
This project exists thanks to many friends who helped me playtest and rethink along the way.
I'd also like to thank mwbraynt for making the project that gave me the initial direction as to how to use a clicked grid: https://github.com/mwbryant/logic_management_game/tree/master 
