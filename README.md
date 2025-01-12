# maku (膜)

- Swiss army knife for image (+video?) manipulation in the AI era.
- This project is purely for personal learning of the GPU + WASM + Rust + Image AI.

### Milestone 1
Simple command line utility with composable GL shader filter + node-based editor to edit filters on web.

- Web UI to compose filters with preview of each block
- GL shader blocks, utility blocks(math)
- Full native mode / Full WASM mode
- Rust native filter command
- Composable filters(composition, reusability)

TODO:

- [ ] Simple node manipulation on Rust
- [ ] Web UI with React
- [ ] Add GPU-related functionality on Rust with three-d crate
- [ ] Add the concept of "composition"

### Milestone2
- Integrate simple machine learning. Change the API to be more protocol based.(Some protobuf based API protocol, we can implement blocks in any language)

### Milestone3
- API server mode like ComfyUI (Run web editor with native mode via API server?)
- Integrate with Python, and StableDiffusion

### To release
- Instructions on how to build the tool
- An examples folder that uses it to convert an image.
- Make the readme page and a marketing page.

