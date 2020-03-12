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

Compiling and running is straightforward and can be done via cargo.

```
git clone https://github.com/WillFarris/ecosimulation
cd ecosimulation
cargo run
```

## Other work on this topic

This project was heavily inspired by Sebastian Lague's [Simulating an Ecosystem](https://www.youtube.com/watch?v=r_It_X7v-1E) video from his Coding Adventure series. While the topic and many features of the game were inspired by the video, all code written was our own.

We didn't do much else in the way of research for this project, but there are plenty of other projects on the web on the topic. Here are 