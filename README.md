# ecosimulation
An ecosystem simulation written in Rust for a term project

## Authors
Will Farris <wfarris@pdx.edu>,
Maggie Haddon <mahadd29@pdx.edu>

## Project Description
This project is an ecosystem simulation written in Rust. When the simulation starts, several critters and food sources populate the map (critters are in pink and red, food is in dark green). Critters move, eat, and mate.  Moving decreases energy, and eating increases it. If a critter runs out of energy completely, it dies. The goal of the project was to create a simple model of how a species interacts with its environment across generations. The average genetic traits of the population are displayed on the screen, so a user can observe how these traits change over time as the species evolves.

Critters have three genetic traits that can be passed down from parent to child: speed, size, and eyesight.
Rate of energy consumption is proportional to speed. Critters move in straight lines, and observe their surroundings withing their eyesight radius. Critters move towards food when they can see it, and when they are "in the mood" they move towards a mate when one is in their radius of sight. When critters mate, the child critter derives its traits from both parents. After mating, the parents no longer want to mate for a short period of time.

Food sources have a set location and maximum size.  The available energy in each plant is based on its size. When they are eaten, plants disappear.  Periodically, eaten plants will regrow, beginning at a very small size and continuing to grow until they hit their maximum size.

## How to build and run the project
Compiling and running is straightforward and can be done via `cargo run`. All libraries should be downloaded automatically.
Due to issues with ggez itself, this project will not run under Wayland. Passing the environment variable `WINIT_UNIX_BACKEND=x11` before running the program fixes this issue, i.e.
`WINIT_UNIX_BACKEND=x11; cargo run`

To explicitly download and run the latest version of the project from GitHub:
```
git clone https://github.com/WillFarris/ecosimulation
cd ecosimulation
cargo run
```
## Testing
Unit testing our project proved difficult for the main game logic, and was mostly done within the math helper functions in `src/math.rs`. We wrote several unit tests to verify the math functions do what is expected.

Because of the randomness of the simulation, unit tests for the main game logic was difficult. For example, the code to mate two critters was straightforward, but produced a random output that would have been difficult to test with assert macros. Most of the functionality of the project is time-dependent, so our testing involved running the program, observing the population over time, and tuning parameters to yield a more interesting result. We didn't think, however, to record intermediate footage during the tuning process.
A demo showing the project is functional can be seen [here](https://youtu.be/eDj6maaKUwE)

## What worked, what didn't, future improvements and general satisfaction
Due to time constraints, we didn't come close to our "final vision" for the project, but we were able to implement all core functionality that sets the groundwork for the project. We have a working model of a species that eats, moves and breeds, and we have a way to visualize change in the population over time. There is room in the future to add another species that acts as a predator, and more genes could be added.
We plan to continue work on this as a side project, and in the future want to make it look a little more aesthetically pleasing with refactored code, sprites, and some cleaner animations. For now however, we are satisfied with the result of the project.
Implementation generally went smoothly other than some issues fighting the borrow checker. While we ran into some issues with the borrow checker we didn't have many issues getting things working.

## Imspiration for the project
This project was heavily inspired by Sebastian Lague's [Simulating an Ecosystem](https://www.youtube.com/watch?v=r_It_X7v-1E) video from his Coding Adventure series. While the topic and many features of the game were inspired by the video, all code written was our own.
We didn't do much else in the way of research for this project, but there are plenty of other projects on the web on the topic. Here are 