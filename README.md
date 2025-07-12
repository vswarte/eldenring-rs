# FromSoftware-rs 🔩  From Software runtime rust bindings
Rust bindings to facilitate mod creation for From Software games.

[![Build Status](https://github.com/vswarte/eldenring-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/vswarte/eldenring-rs/actions)
![Crates.io License](https://img.shields.io/crates/l/eldenring)

<details>

<summary>Example Elden Ring mod code: render debug line</summary>

Your project's Cargo.toml should contain the following lines (substitute the paths for where you unpacked the library):
```toml
[lib]
crate-type = ["cdylib"] # Compiles a DLL that will run in the game

[dependencies.eldenring]
path = "dependencies/fromsoftware-rs/crates/eldenring"
[dependencies.eldenring-util]
path = "dependencies/fromsoftware-rs/crates/eldenring-util"
[dependencies.fromsoftware-shared]
path = "dependencies/fromsoftware-rs/crates/shared"
[dependencies.nalgebra-glm]
version = "0.19.0"
```

```rust
use std::time::Duration;

use eldenring::{
    cs::{CSTaskImp, RendMan, WorldChrMan},
    fd4::FD4TaskData,
    position::PositionDelta,
};
use eldenring_util::{
    program::Program,
    ez_draw::CSEzDrawExt, singleton::get_instance, system::wait_for_system_init, task::CSTaskImpExt,
};

use fromsoftware_shared::{
    FSVector4,
};
use nalgebra_glm as glm;

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    // Check if we're attaching to the game
    if reason == 1 {
        // Kick off new thread.
        std::thread::spawn(|| {
            // Wait for game (current program we're injected into) to boot up.
            wait_for_system_init(&Program::current(), Duration::MAX).expect("Could not await system init.");

            // Retrieve games task runner.
            let cs_task = get_instance::<CSTaskImp>().unwrap().unwrap();

            // Register a new task with the game to happen every frame during the gameloops
            // ChrIns_PostPhysics phase because all the physics calculations have ran at this
            // point.
            cs_task.run_recurring(
                // The registered task will be our closure.
                |_: &FD4TaskData| {
                    // Grab the debug ez draw from RendMan if it's available. Bail otherwise.
                    let Some(ez_draw) = get_instance::<RendMan>()
                        .expect("No reflection data for RendMan")
                        .map(|r| r.debug_ez_draw.as_ref())
                    else {
                        return;
                    };

                    // Grab the main player from WorldChrMan if it's available. Bail otherwise.
                    let Some(player) = get_instance::<WorldChrMan>()
                        .expect("No reflection data for WorldChrMan")
                        .map(|w| w.main_player.as_ref())
                        .flatten()
                    else {
                        return;
                    };

                    // Grab physics module from player.
                    let physics = &player.chr_ins.module_container.physics;

                    // Make a directional vector that points forward following the players
                    // rotation.
                    let directional_vector = {
                        let forward = glm::vec3(0.0, 0.0, -1.0);
                        glm::quat_rotate_vec3(&physics.orientation.into(), &forward)
                    };

                    // Set color for the to-be-rendered line.
                    ez_draw.set_color(&FSVector4(0.0, 0.0, 1.0, 1.0));

                    // Draw the line from the players position to a meter in front of the player.
                    ez_draw.draw_line(
                        &physics.position,
                        &(physics.position
                            + PositionDelta(
                            directional_vector.x,
                            directional_vector.y,
                            directional_vector.z,
                        )),
                    );
                },
                eldenring::cs::CSTaskGroupIndex::ChrIns_PostPhysics,
            );
        });
    }

    // Signal that DllMain executed successfully
    true
}
```

Result:
![Debug line rendered by example mode code](img/example-mod-debug-line.png)

</details>

# Project structure (crates)
 - `crates/eldenring` Contains the definitions for the elden ring structures. [![Crates.io](https://img.shields.io/crates/v/eldenring.svg?label=eldenring)](https://crates.io/crates/eldenring) [![Documentation](https://docs.rs/eldenring/badge.svg)](https://docs.rs/eldenring)
 - `crates/nightreign` Contains the definitions for the nightreign structures. [![Crates.io](https://img.shields.io/crates/v/nightreign.svg?label=nightreign)](https://crates.io/crates/nightreign) [![Documentation](https://docs.rs/nightreign/badge.svg)](https://docs.rs/nightreign)
 - `crates/util` Provides helper methods for common stuff. [![Crates.io](https://img.shields.io/crates/v/eldenring-util.svg?label=eldenring-util)](https://crates.io/crates/eldenring-util) [![Documentation](https://docs.rs/eldenring-util/badge.svg)](https://docs.rs/eldenring-util) 
 - `crates/dlrf` Defines a trait and exports a macro for interacting with the games reflection system. [![Crates.io](https://img.shields.io/crates/v/eldenring-dlrf.svg?label=eldenring-dlrf)](https://crates.io/crates/eldenring-dlrf)  [![Documentation](https://docs.rs/eldenring-dlrf/badge.svg)](https://docs.rs/eldenring-dlrf) 
 - `crates/dlrf/derive` Defines the derive macro for implementing the DLRF trait on types. **Do not depend on this directly since the macro is reexported through `eldenring-dlrf`**. [![Crates.io](https://img.shields.io/crates/v/eldenring-dlrf-derive.svg?label=eldenring-dlrf-derive)](https://crates.io/crates/eldenring-dlrf-derive)  [![Documentation](https://docs.rs/eldenring-dlrf-derive/badge.svg)](https://docs.rs/eldenring-dlrf-derive) 

# Credits (aside listed contributors to this repository)
 - Tremwil (for the arxan code restoration disabler, vtable-rs and a few other boilerplate-y things as well as implementing the initial FD4 singleton finder for TGA that I appropriated).
 - Dasaav (for [libER](https://github.com/Dasaav-dsv/libER) and heaps of engine-related structures).
 - Sfix (for coming up with the FD4 singleton finder approach at all).
 - Yui (for some structures as well as AOBs and hinting at some logic existing in the binary).
 - Vawser (and probably many more) (for hosting the param defs used with the param struct generator).

(Have you contributed to TGA in some manner and does this repository have your work in it? Reach out to @chainfailure on Discord for proper credit disclosure).
