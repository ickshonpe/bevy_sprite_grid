# bevy sprite grid

Extremely basic crate for drawing sprites in a grid.

It has one plugin, with one system. A SpriteGrid is a single entity with a SpriteGrid component and, like most Bevy renderables, Transform, GlobalTransform, and Visibility components.

Drawing is done by the built-in Bevy sprite renderer. No intermediate sprite entities are created.

It supports regular Bevy sprites, sprites from a TextureAtlas, or a mix of them, or it can just draw untextured quads if all you need is a checkerboard or something.

Spawning a SpriteGridBundle is similar to spawning an ordinary Bevy SpriteBundle. 
Transformations applied to the SpriteGrid entity should correctly propagate to its sprites.

No proper documentation, but there are quite a lot of examples in the /examples dir. 

Performance should be fine, and more than enough for most practical purposes. It's not optimised at all though, except for culling of off screen sprites. It shouldn't matter how many sprites a SpriteGrid has (even tens of millions of sprites), as long as most of them are out of view.

The math is a bit of a mess and probably includes a few unnecessary operations. 
The SpriteGrid API is a work in progress as well.

If you are doing a tile based 2D game you almost certainly want to use [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) instead, which is well supported, has great performance, and lots of features.







