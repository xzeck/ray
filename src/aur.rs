use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufRead};
use std::process::{Child, Command, Stdio};
use colored::*;

use serde::Deserialize;
extern crate serde_json;
extern crate serde;



// paths
const PACMAN_BIN: &str = "/usr/bin/pacman";
const MAKEPKG_BIN: &str = "/usr/bin/makepkg";
const TAR_BIN: &str = "/usr/bin/tar";
const RAY_TMP: &str = "/tmp/raytmp/";
const BASE_URL: &str = "https://aur.archlinux.org";


// Response Structure
#[derive(Debug, Deserialize)]
struct AurResponse {
    #[serde(skip_deserializing)]
    version: i32,

    #[serde(skip_deserializing)]
    #[serde(rename(deserialize = "type"))]
    query_type: String,

    #[serde(skip_deserializing)]
    #[serde(rename(deserialize = "resultcount"))]
    result_count: i32,

    results: Vec<PackageData>,
}

// Required data structure
#[derive(Debug, Deserialize, Clone)]
pub struct PackageData {

    #[serde(rename(deserialize = "ID"))]
    pub id: i64,

    #[serde(rename(deserialize = "Name"))]
    pub name: Option<String>,

    #[serde(rename(deserialize = "PackageBaseID"))]
    pub package_base_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename(deserialize = "PackageBase"))]
    pub package_base: Option<String>,

    #[serde(rename(deserialize = "Version"))]
    pub version: Option<String>,

    #[serde(rename(deserialize = "Description"))]
    pub description: Option<String>,

    #[serde(rename(deserialize = "URL"))]
    pub url: Option<String>,

    #[serde(skip_deserializing)]
    #[serde(default)]
    #[serde(rename(deserialize = "NumVotes"))]
    pub num_votes: i64,

    #[serde(skip_deserializing)]
    #[serde(rename(deserialize = "Popularity"))]
    pub popularity: f32,

    #[serde(rename(deserialize = "OutOfDate"))]
    pub out_of_date: Option<i32>,

    #[serde(default)]
    #[serde(rename(deserialize = "Maintainer"))]
    pub maintainer: Option<String>,

    #[serde(rename(deserialize = "FirstSubmitted"))]
    pub first_submitted: i64,

    #[serde(skip_deserializing)]
    #[serde(default)]
    #[serde(rename(deserialize = "LastModified"))]
    pub last_modified: i64,

    #[serde(rename(deserialize = "URLPath"))]
    pub url_path: Option<String>,
}

pub async fn search_aur(package: String) -> Result<Vec<PackageData>, Box<dyn std::error::Error>> {
    let mut url: String = "https://aur.archlinux.org/rpc/?v=5&type=search&arg=".to_owned();
    let package = package.to_owned();
    url.push_str(&package);

    let client = reqwest::Client::new();
    let resp = client.get(url).send().await?;

    let resp_json = resp.json::<AurResponse>().await.unwrap();

    let mut package_data = Vec::new();

    for packages in resp_json.results {
        package_data.push(packages);
    }

    Ok(package_data)
}


pub async fn download_file(filepath: &String, url: String) -> Result<(), Box<dyn Error>> {
    // create file
    // create_directory(filepath);
    // Get data

    let mut file = match File::create(&filepath) {
        Err(why) => {
            panic!("Could not create file {}", why);
        },
        Ok(file) => file,
    };

    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    
    file.write_all(&content[..])?;
    Ok(())

}

pub async fn write_to_stdout(process:  &mut Child) {
    let stdout = process.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();

    for line in stdout_lines {
        println!("{:?}", line);
    }
}
pub async fn unpack_file(tar_location: &String, unpack_location: &String) -> Result<(), Box<dyn Error>>{
    // Unpack file

    let mut tar_command = Command::new("tar")
                                        .arg("-xvzf")
                                        .arg(tar_location)
                                        .arg("-C").arg("/tmp/raytmp/builds/")
                                        .stdout(Stdio::piped())
                                        .spawn().unwrap();

    let mut makepkg_command = Command::new("makepkg")
                                            .arg("-sri")
                                            .arg("--noconfirm")
                                            .current_dir(unpack_location)
                                            .stdout(Stdio::piped())
                                            .spawn()
                                            .unwrap();

    write_to_stdout(&mut tar_command).await;
    write_to_stdout(&mut makepkg_command).await;
    Ok(())
}


pub async fn install(packages_to_install: Vec<PackageData>) -> Result<(), Box<dyn Error>>{

    let build_dir = concat_string!(RAY_TMP, "builds");

    // Create build directory
    // /tmp/raytmp/builds
    match create_directory(&build_dir) {
        Ok(()) => {

        },
        Err(_) => {
            dbg!("Error creating file");
            panic!();
        }
    }

    println!();
    println!("Starting Download");
    for package_data in packages_to_install {

        // Get name of the package
        let package_name = package_data.name.as_ref().unwrap();
        
        // Where package data will be downloaded
        // /tmp/raytmp/builds/<package-name>/

        let unpack_location = concat_string!(build_dir, "/", package_name);

        // name of tar
        // /tmp/raytmp/builds/<package-name>/<package-name.tar.gz>
        let tar_name_path = concat_string!(build_dir, "/", package_name, ".tar.gz");
        
        // get the AUR url
        let package_url = package_data.url_path.as_ref().unwrap();
        
        // generate full URL
        let download_url = concat_string!(BASE_URL,  package_url);

        
        // Download file
        match download_file(&tar_name_path, download_url).await {
            Ok(_) => {
                println!("{}-{} - Downloaded", package_data.name.unwrap().bright_green(), package_data.version.unwrap().bright_cyan());
                unpack_file(&tar_name_path, &unpack_location).await;
            },
            Err(why) => {
                println!("Error Downloading - {} - Error", package_data.name.unwrap().bright_red());
                println!("Error Downloading {}", &why);
                continue;
            }
        }

        fs::remove_file(tar_name_path);

    }
   
    Ok(())

}

fn create_directory(filepath: &String) ->std::io::Result<()>{

    if fs::metadata(&filepath).is_ok() {
        Ok(())
    }
    else {
        let path = Path::new(filepath.as_str());

        fs::create_dir_all(path).expect("Could not create file");
        let permissions = fs::Permissions::from_mode(0o755);
        fs::set_permissions(path, permissions)?;
        Ok(())
    }

}
