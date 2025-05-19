# My NES Emulator written in Rust

## Why
- I have wanted to build an emulator for a long time, it is simultaneously very easy (just implement the instructions, right?) and very hard (implement the instructions + everything else involved)
- It was a good project to work on both Rust fundamentals and understanding low level CPU functionality

## How
- The whole emulator is written as a crate such that it can be attached to different display-ers. Currently there is only one, but if I wanted to port it to WASM or TUI then I could do that
- It aims to replicate the whole CPU as accurately as possible, including cycle synchronization with the PPU and interrupts between the processors

## Current state of the project
- I have got Donkey Kong running briefly
- Backgrounds are implemented, foreground sprites are not yet (not my top priority yet)
- I am currently refactoring the logging system to use generics and be far more adaptable
- Controllers are not yet implemented but the framework is there

### Future plans
- Add support for other memory mappers to support more ROMs
- Add audio (harder than it seems)
- Add a usable gui with EGUI to change settings on the fly and save state
