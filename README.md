# bevy sprite grid

Draws sprites in a grid.

Extremely basic tile map crate.

You probably want to use [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) which is well supported, has great performance, and a legion of features.

The only positive thing to be said about this crate is that is really, really simple.

It has one plugin, with one system. A SpriteGrid is a single entity with a SpriteGrid component and, like most Bevy renderables, Transform, GlobalTransform, and Visibility components.

Drawing is done by the built-in Bevy sprite renderer. No intermediate sprite entities are created.

It supports regular Bevy sprites, sprites from a TextureAtlas, or a mix of them, or it can just draw untextured quads if all you need is a checkerboard or something.

Spawning a SpriteGridBundle is similar to spawning an ordinary Bevy SpriteBundle. 
Transformations applied to the SpriteGrid entity should correctly propagate to its sprites.

No proper documentation yet, but there are quite a lot of examples in the /examples dir. It is an incredibly simple crate.

Performance should be fine, and more than enough for most practical purposes. It's not optimised though, and there are lots of ways to make it quite a bit more efficient.









