# Param Generator

Generates rust structs from specified paramdef XML files.
This tool ignores any paramdef fields with a maximum version specified since we assume we're always building for the latest version of the game.

To rebuild `crates/game`'s param.rs, invoke the following from the repo's root:
`$ cargo run --bin param-generator -- --input tools/param-generator/params/ --output crates/game/src/param.rs`
