# maku (膜)

This is my personal project to re-learn WebAssembly, Rust, OpenGL, WebGL with a small command line utility.
The command line utility (for now) accepts a GLSL shader text and an input image and an output effect-applied image that runs the shader on GPU. 
The command line run on your PC natively, and it's library is build for WASM too.
And you can run the GLSL testing web app on the demo website of this repo.

The initial goal:

- Really useful image filter command line tool (Try to keep the implementation very simple)
- Composable project structure for reusability (Group, Variable)
- Web UI to edit projects
- (Machine learning filters)
