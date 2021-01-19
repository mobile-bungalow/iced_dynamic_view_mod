# dynamic view module iced example

## What is this?
Some people were wondering about how to speed up incremental iced builds. Because iced uses the elm architecture
it's state is nicely encapsulated and modifiable in the the runtimes calls to `application::view`. We can leverage this
to dynamically recompile and relink just the view function, this could save us time by avoiding relinking larger dependencies
which might include tungstenite, or libraries that make heavy use of proc macros.

## how do I run it?
1. build the dylib in `view_mod` with `cargo build`.
2. call `cargo run` in the main crate.
3. make changes to the view mod, build it again, do anything which sends a message, watch the UI update.

## Does it really improve iteration time?
No, not in most cases, but it really could given a few minor changes.
1. with the addition of a dynamic backend to iced we could pass a `box<dyn Renderer>` to the dylib and not have to
relink wgpu, gfx, lyon, etc. every time we rebuild the dylib.
2. Iced is lazy in it's calls to view, so refreshing requires the view to be updated, writing a subscriber that watches file events
and calls `dymod::reload` would also be ideal.
3. this is actually a lot of hassle.

## Goals
ideally a dynamic backen to iced would build in less that 2 seconds allowing for really HTML equivalent iteration speeds.
