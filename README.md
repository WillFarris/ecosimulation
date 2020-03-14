# ecosimulation
An ecosystem simulation written in Rust for a term project

## Authors
Will Farris <wfarris@pdx.edu>,
Maggie Haddon <mahadd29@pdx.edu>

## Project Description
Demo at <https://youtu.be/eDj6maaKUwE>
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
Testing with 

## What worked & what could be improved
This program basically works as expected.  Due to time contraints, we didn't implement all of the features that we wanted to, but all of the core functionality that we expected to finish are done.  We purposely planned to add more features than time allowed, and those features provide ideas for what to add next.  This is a project that we are both excited to work on in the future as a side project.

Some improvements/additions that we hope to add are predators that eat the prey critters, more detailed animation, and flocking algorithms to reduce random movement.

## Other work on this topic

This project was heavily inspired by Sebastian Lague's [Simulating an Ecosystem](https://www.youtube.com/watch?v=r_It_X7v-1E) video from his Coding Adventure series. While the topic and many features of the game were inspired by the video, all code written was our own.

We didn't do much else in the way of research for this project, but there are plenty of other projects on the web on the topic. Here are 