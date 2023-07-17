# Zigzag
Zigzag is a chess engine built primarily with Zig (hence the creative name), whose primary components are stored in `zigzag/core`. This monorepo is intended to store both the essential components of the engine, as well as related tools and interfaces.

This project was largely inspired by Sebastian Lague's excellent series on implementing a chess engine in C#, which is an excellent introduction to chess programming and some key ideas within. Go watch it [here](https://www.youtube.com/watch?v=_vqlIPDR2TU&list=PLFt_AvWsXl0cvHyu32ajwh2qU1i6hl77c&pp=iAQB).

# Components
## Core
`zigzag/core` is the underlying essential base of Zigzag, providing essential functionality to all other components. It doesn't provide C ABI compatibility (currently), so other components are expected to provide Zig wrappers where necessary.

# Potential Future Components
- A Python library via the CPython ABI (3.11.4+).
- A JS/Zigzag wrapper via `node-ffi`.
- Other GUI and analysis components.
