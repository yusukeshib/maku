# maku (膜)

The command line utility (for now) accepts a GLSL shader text and an input image and an output effect-applied image that runs the shader on GPU. 

This is my personal project to re-learn WebAssembly, Rust, OpenGL, WebGL with a small command line utility.

The command line run on your PC natively, and it's library is build for WASM too.
And you can run the GLSL testing web app on the demo website of this repo.

## Initial functionalities to aim

- [ ] Study the design of ImageMagick command lines and improve it
- [ ] Study the design of OpenCV and think about it
- [ ] Given an input image, and one GLSL shader file, output an image with specified output path.
- [ ] Preview filtered image on a app window.
- [ ] Watch option of GLSL shader file, to develop GLSL efficiently with the app preview window.
- [ ] Chaining filters

## In the future

- [ ] Support various filter types including AI models like StableDiffusion

