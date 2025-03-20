# Binary Mapper

## Description
Used for finding statics and functions which may require nested matching to follow IBOs. Used to generate bindings and RVA files when new versions of Elden Ring are introduced.
1. Build provides an EXE
1. EXE requires arguments or envs
    1. env MAPPER_GAME_EXE or --exe
        1. Expects eldenring.exe full path
    1. env MAPPER_OUTPUT_FORMAT or --output
        1. Expects print or rust
    1. env MAPPER_PROFILE or --profile
        1. Expects a TOML
        1. TOML contains AOB patterns supposedly matching within the .text section of the EXE
        1. Toml format should match the following: 
```toml
[[patterns]] # Required dict name
pattern = "55 a0 ?" # https://docs.rs/pelite/latest/pelite/pattern/fn.parse.html
captures = ["capture1", "capture2"] # These are just example names

[[patterns]]
pattern = "8B ?? ?? ?? ?? 89"
captures = ["capture3", "capture4"]
```