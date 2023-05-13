# Image Resizer

Image Resizer is a Rust program that resizes an image file while maintaining its aspect ratio. The program accepts an image file as a command-line argument, resizes it to fit within a maximum size of 800x600 pixels, and saves the resized image with a new filename.

## Requirements

- Rust (nightly or stable)

## Usage

1. Clone this repository or download the source code.

2. Navigate to the project directory.

3. Build the project using Cargo:

   ```shell
   cargo build --release


## Run the Program
Run the program, providing the path to the image file as a command-line argument:

cargo run --release -- <image_file>
Replace <image_file> with the path to the image file you want to resize.

The program will resize the image while maintaining its aspect ratio and save the resized image as <original_filename>_800_600.jpg in the same directory as the original image.
