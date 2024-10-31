# mvph - Move Photos and Videos
mvph is a Rust command-line tool designed to organize and copy your photos and videos from one folder to another. It uses the creation or modification date of each file to create a structured directory hierarchy in the destination path.

## Features
Organizes files by date: Creates a directory structure based on the year, month, and day of the file’s creation or modification date.
Multi-threaded processing: Utilizes multi-threading to speed up the file copying process.
Handles large files efficiently: Optimized for handling large files, making it suitable for high-resolution photos and videos.

## Installation
To install mvph, you need to have Rust installed. Then, you can build the project using cargo:

```
git clone https://github.com/yourusername/mvph.git
cd mvph
cargo build --release
```

## Usage
To use mvph, run the following command:

```
./target/release/mvph <source_directory> <destination_directory>
```

Replace <source_directory> with the path to the folder containing your photos and videos, and <destination_directory> with the path where you want the organized files to be copied.

## Example

```
./target/release/mvph /path/to/source /path/to/destination
```

This command will copy all photos and videos from /path/to/source to /path/to/destination, organizing them into folders based on their creation or modification dates.

## Code Overview
Here’s a brief overview of the main parts of the code:

Gather Metadata and Sort Files: Reads the source directory, gathers metadata, and sorts files into a HashMap where the key is the date string and the value is a vector of file paths.
Perform Multi-threaded File Copy: Uses rayon to parallelize the directory creation and file copying processes.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License
This project is licensed under the MIT License.


