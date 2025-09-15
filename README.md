# Rice UI

An attempt at a Rust-based, WGPU-powered, scriptable, animatable UI library.

Inspired by the [Clay](https://github.com/nicbarker/clay) UI layout library.

## Project Crates

- [`examples`](./examples): usage examples
- [`rice-dom`](./rice-dom): DOM management
- [`rice-fmt`](./rice-fmt): formatter for Rice DSL
- [`rice-grammar`](./rice-grammar): treesitter grammar for Rice DSL
- [`rice-layout`](./rice-layout): layout computation framework
- [`rice-parser`](./rice-parser): parse Rice DSL into DOM
- [`rice-ui`](./rice-ui): main crate
- [`rice-wgpu`](./rice-wgpu): WGPU rendering for Rice UI

## Roadmap

- [ ] layout computation framework
  - [x] fixed size components
  - [x] growable size components
  - [ ] text components (wrapping)
  - [x] alignment
  - [x] margin & padding
  - [x] min/max height/width for fit & expand sizes
  - [ ] grid
- [ ] WGPU rendering
  - [x] basic rendering
  - [ ] common styles (easy theme / style overrides)
  - [ ] animation support (outside of manual UI update)
- [ ] Scripting
  - [ ] declare layouts in Rice DSL + hot reload
  - [ ] define logic in Lua (or the likes) + hot reload
