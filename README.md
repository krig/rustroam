# rustroam

Voxel world renderer demo

Mainly a tool for practicing Rust. My initial plan is to implement
something similar to [Roam][roam] which is a very simple voxel world
renderer I wrote in C a while back.

[roam]: https://github.com/krig/roam

## Concept

My basic plan for this voxel generator is to try to do a low poly look
with basically single-color blocks, and do some full-screen post
processing.

## TODO

* Game loop
* Mesh building
* Chunk generation / cache
* Chunk saving / loading
* Threaded save / load / generation
* Landscape shader
* Player controls
* Noise
* Trees
* Non-landscape things?
* Audio?

