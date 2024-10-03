# maku (膜)

The command line utility (for now) accepts a GLSL shader text and an input image and an output effect-applied image that runs the shader on GPU. 

This is my personal project to re-learn WebAssembly, Rust, OpenGL, WebGL with a small command line utility.

The command line run on your PC natively, and it's library is build for WASM too.
And you can run the GLSL testing web app on the demo website of this repo.

## Functionality

- Command line
	- Generate a new image
	- Preview screen
	- Watch input files
- Node editor to edit project on web
	- Variables
	- Math filters
	- Machine learning filters
- Examples
    - One of filters is face detector machine learning filter, and the detected face rectangles in the input image are pixelated with a pixelated filter.


