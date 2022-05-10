# Description
This program is a simple demonstration and benchmark of the AES-CBC encryption algorithm implemented in Rust 1.60 using Cargo as the package manager.

# Expected Behavior
Upon running the program, be patient and allow it a few seconds to complete its first encryption and decryption pass. The time this takes will vary from system to system. The program will run 20 passes of encryption and decryption on a target text (`.txt`) file and print the times each time it completes a pass. It will then print an average encryption and decryption times and the best encryption and decryption times upon completing all 20 runs.

# How to Run
## Installation
In order to run this program, you will need to have Rust 1.60 installed and the corresponding version of Cargo. You can follow the installation instructions [here](https://www.rust-lang.org/tools/install). The project was built and ran on Linux, so that is what the rest of the guide will reference for execution.

## Compilation and Execution
In order to compile and run the program, first, you must edit the `main.rs` file in the `aes/src` folder to use the correct file path to run the encryption and decryption processes on. Replace the file path with the **full path** to your file in the `message` variable on line 102. Make sure to save the file upon making your changes. 
<br><br>
Next, compile the code. To do this, navigate to the `aes` directory and run the command `cargo build --release` in the terminal to build the release executable (meaning that the debugging flags are stripped out and it is compiled with optimizations enabled). Running the build command will fetch and compile all necessary dependency packages denoted in the `Cargo.toml` file and add them to the executable. This will build an executable program file in the `aes/target/release` directory simply called `aes`. 
<br><br>
Finally, execute the program by running `./aes` in the terminal.