use flate2::read::GzDecoder;
use reqwest::blocking::get;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tar::Archive;

fn main() {
    let package_version = "4.9.0";
    let package_url = format!(
        "https://registry.npmjs.org/@openzeppelin/contracts/-/contracts-{}.tgz",
        package_version
    );
    let package_folder = "package/build/contracts/";

    // Send an HTTP GET request to download the package
    let response = get(&package_url).expect("Failed to send request");

    // Check if the response was successful
    if response.status().is_success() {
        // Create a GzDecoder to decompress the gzip archive
        let decoder = GzDecoder::new(response);

        // Create a tar archive from the decoder
        let mut archive = Archive::new(decoder);

        for entry_result in archive.entries().expect("Failed to read tarball entries") {
            let mut entry = entry_result.expect("Failed to process tarball entry");

            // Check if the entry is a file
            if entry.header().entry_type().is_file() {
                // Get the path of the file entry
                let file_path = entry.path().expect("Failed to retrieve directory path");
                let file_name = file_path
                    .to_str()
                    .unwrap()
                    .split("/")
                    .last()
                    .expect("Failed to generate file name");

                if file_path.starts_with(package_folder) {
                    // Perform desired operations on the contracts directory
                    println!("Directory found: {:?}", file_path);
                }

                let output_file_path = file_path.file_name().expect("Failed to retrieve file name");

                // Create the destination path in the current directory
                let destination_path = Path::new("./lol").join(file_name);

                // Extract and copy the file to the destination path
                entry.unpack(destination_path).expect("");
            }
        }
    } else {
        println!(
            "Failed to download package. Status code: {}",
            response.status()
        );
    }
}
