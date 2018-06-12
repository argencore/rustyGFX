# RustyGFX
## Code Status
currently this is a work in progress.
This is a toy program! not anything to be used as is for real work.
The current program motto is if it shouldn't be done crash and so this will probably still crash often.
## Installing RustyGFX
To install RustyGFX you need to have rust installed as well as cargo.
With these installed it is as simple as cloning this repo and then in the root directory of this project typing cargo build --release and cargo run --release to run it.
The program will start up with some example objects and shaders so that if all goes well you will see a cube on the screen with some lighting.
## What RustyGFX does
RustryGFX is a toy rendering engine that displays obj file objects to the screen.
It allows you to experiment with vertex and fragment shaders and watch how changes in them change your scene in real time.
It allows you to move around your object and to switch out objects on the fly.
## How to use RustyGFX
### Basic Controls
* press ENTER/RETURN followed by f to reload the fragment shader
* press ENTER/RETURN followed by v to reload the vertex shader
* press ENTER/RETURN followed by o to reload the object file and reset the currently rendered object to the first.
* press ENTER/RETURN followed by n to get the next object in the file loaded
(NOTE: the objects will wrap around to the first after you reach the last)
* W A S D are used for movement in the x and z while PgUp and PgDn are used for movement in the y
* the arrow keys are used for rotation 
(NOTE: currently movement is tricky because it is based on absolute coordinates instead of relative)

There is a file called vertex_shader.vert this is the file that contains the vertex shader in GLSL. 
Change this file to change the vertex shader and see the results.
Similarly there is a file called fragment_shader.frag this contains the fragment shader in GLSL and you can change this to change the fragment shader.
To see the effects of the changes that you make reload the shaders that you change.
The object names are in the file objectsToLoad.txt to add an object put its name in this file and place the object with the others in the root project directory.
IMPORTANT ONLY USE OBJ FILES AND MAKE SURE THAT THE FILES HAVE TEXTURE COORDINATES THIS MEANS THAT INSIDE THE OBJ FILES YOU SHOULD SEE LINES STARTING WITH vt AND THE LINES THAT START WITH  f should look like #/#/# NOT #//# FAILURE TO DO THIS WILL CURRENTLY RESULT IN THE PROGRAM CRASHING WHEN IT TRIES TO LOAD YOUR FILE.
NOTE INVALID SHADERS WILL ALSO CAUSE CRASHING.

## Contact info
a32@pdx.edu
