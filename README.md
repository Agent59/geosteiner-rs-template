# geosteiner-rs-template
A template for working with the geosteiner C library in Rust

## Installation
**This project does not run on ARM systems like the raspberry pi.**  

&nbsp;&nbsp; 1\. Download this project or click "use this template" to create a repository using this project as a template.  
&nbsp;&nbsp; 2\. Make changes according to your own project. (optional if testing this template)  

With Docker:  
&nbsp;&nbsp; 3\. Modify the Dockerfile to fit your project. (Should work out of the box for the template)  
&nbsp;&nbsp; 4\. Run the Docketfile.

Without Docker:  
&nbsp;&nbsp; 3\. Use `cargo build --features rebuild_c` to build the project and the needed C library.  
&nbsp;&nbsp; 4\. Run the project with `cargo run`.

Debugging:  
To debug the building process delete the target directory with `cargo clean`  
and use `cargo build -vv --features rebuild_c` to rebuild everything.
