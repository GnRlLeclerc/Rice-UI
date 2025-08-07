# Rice UI

An attempt at a Rust-based, WGPU-powered, scriptable, animatable UI library.

Inspired by the [Clay](https://github.com/nicbarker/clay) UI layout library.

## Project Crates

- [`examples`](./examples): usage examples
- [`rice-layout`](./rice-layout): layout computation framework
- [`rice-ui`](./rice-ui): main crate
- [`rice-wgpu`](./rice-wgpu): WGPU rendering for Rice UI

## Roadmap

- [ ] layout computation framework
  - [x] fixed size components
  - [ ] growable size components
  - [ ] text components (wrapping)
  - [x] alignment
  - [x] margin & padding
  - [ ] grid
- [ ] WGPU rendering
  - [ ] basic rendering
  - [ ] common styles (easy theme / style overrides)
  - [ ] animation support (outside of manual UI update)
- [ ] Scripting
  - [ ] declare layouts in JSON (or the likes) + hot reload
  - [ ] define logic in Lua (or the likes) + hot reload
