use ethers_contract_abigen::Abigen;
use flate2::read::GzDecoder;
use reqwest::blocking::get;
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use tar::Archive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let abi_folder_path: &str = "abis/";
    let library_folder_path = "openzeppelin-rs/src";

    let mut library_contracts_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}/contracts.rs", library_folder_path))?;
    library_contracts_file.seek(SeekFrom::Start(0))?;
    library_contracts_file.set_len(0)?;

    library_contracts_file.write_all(format!("#![allow(non_snake_case)]\n\n").as_bytes())?;

    let mut library_lib_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}/lib.rs", library_folder_path))?;
    library_lib_file.seek(SeekFrom::Start(0))?;
    library_lib_file.set_len(0)?;

    library_lib_file.write_all(format!("#![allow(non_snake_case)]\n\n").as_bytes())?;
    library_lib_file.write_all(format!("mod contracts;\n\n").as_bytes())?;

    create_folder_if_not_exists(format!("{}/contracts", library_folder_path).as_str())?;

    if let Ok(entries) = fs::read_dir(abi_folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(abi_contract_file) = entry.file_name().to_str() {
                    let abi_contract_path = abi_folder_path.to_string() + abi_contract_file;

                    let contract_name = match abi_contract_file.strip_suffix(".json") {
                        Some(name) => name,
                        None => abi_contract_file,
                    };

                    let out_file =
                        format!("{}/contracts/{}.rs", library_folder_path, contract_name);

                    library_contracts_file
                        .write_all(format!("pub mod {};\n", contract_name).as_bytes())?;
                    library_lib_file.write_all(
                        format!("pub use contracts::{}::{};\n", contract_name, contract_name)
                            .as_bytes(),
                    )?;

                    Abigen::new(contract_name, abi_contract_path)?
                        .generate()?
                        .write_to_file(out_file)?;
                }
            }
        }
    } else {
        println!("Failed to read directory");
    }

    Ok(())
}

fn create_folder_if_not_exists(folder_path: &str) -> std::io::Result<()> {
    if !fs::metadata(folder_path).is_ok() {
        fs::create_dir(folder_path)?;
        println!("Folder '{}' created", folder_path);
    } else {
        println!("Folder '{}' already exists", folder_path);
    }
    Ok(())
}

// fn fetch_contracts() {
//     let package_version = "4.9.0";
//     let package_url = format!(
//         "https://registry.npmjs.org/@openzeppelin/contracts/-/contracts-{}.tgz",
//         package_version
//     );
//     let package_folder = "package/build/contracts/";

//     // Send an HTTP GET request to download the package
//     let response = get(&package_url).expect("Failed to send request");

//     // Check if the response was successful
//     if response.status().is_success() {
//         // Create a GzDecoder to decompress the gzip archive
//         let decoder = GzDecoder::new(response);

//         // Create a tar archive from the decoder
//         let mut archive = Archive::new(decoder);

//         // Create the destination path in the current directory
//         create_folder_if_not_exists("contracts").expect("Failed to create contracts folder");

//         for entry_result in archive.entries().expect("Failed to read tarball entries") {
//             let mut entry = entry_result.expect("Failed to process tarball entry");

//             // Check if the entry is a file
//             if entry.header().entry_type().is_file() {
//                 // Get the path of the file entry
//                 let file_path = entry.path().expect("Failed to retrieve directory path");
//                 let file_name = file_path
//                     .to_str()
//                     .unwrap()
//                     .split("/")
//                     .last()
//                     .expect("Failed to generate file name");

//                 if file_path.starts_with(package_folder) {
//                     // Perform desired operations on the contracts directory
//                     println!("Populated: {:?}", file_path);

//                     let destination_path = Path::new("gen/contracts").join(file_name);

//                     // Extract and copy the file to the destination path
//                     entry.unpack(destination_path).expect("");
//                 }
//             }
//         }
//     } else {
//         println!(
//             "Failed to download package. Status code: {}",
//             response.status()
//         );
//     }
// }
