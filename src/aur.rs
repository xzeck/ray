use std::fs;
use std::os::unix::fs::PermissionsExt;

use serde::Deserialize;
extern crate serde_json;
extern crate serde;



// paths
const PACMAN_BIN: &str = "/usr/bin/pacman";
const MAKEPKG_BIN: &str = "/usr/bin/makepkg";
const TAR_BIN: &str = "/usr/bin/tar";
const RAY_TMP: &str = "/tmp/raytmp";
const BASE_URL: &str = "https://aur.archlinux.org";


// Response Structure
#[derive(Debug, Deserialize)]
struct AurResponse {
    version: i32,

    #[serde(rename(deserialize = "type"))]
    query_type: String,

    #[serde(rename(deserialize = "resultcount"))]
    result_count: i32,

    results: Vec<PackageData>,
}

// Required data structure
#[derive(Debug, Deserialize)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "Description"))]
    pub description: Option<String>,

    #[serde(rename(deserialize = "URL"))]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename(deserialize = "NumVotes"))]
    pub num_votes: i64,

    #[serde(rename(deserialize = "Popularity"))]
    pub popularity: f32,

    #[serde(rename(deserialize = "OutOfDate"))]
    pub out_of_date: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename(deserialize = "Maintainer"))]
    pub maintainer: Option<String>,

    #[serde(rename(deserialize = "FirstSubmitted"))]
    pub first_submitted: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename(deserialize = "LastModified"))]
    pub last_modified: i64,

    #[serde(rename(deserialize = "URLPath"))]
    pub url_path: Option<String>,
}

#[tokio::main]
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

    // let package: Vec<PackageData> = Vec::new();

    // Ok(package)
}

pub async fn download_file(filepath: String, url: String) -> std::io::Result<()> {
    // create file
    // create_directory(filepath);
    // Get data
    
    // Write data to file

    Ok(())

}

pub async fn install(package_data: PackageData) {
    // DONE: creat folder with 0755 permission
    // check for errors

    // generate tar location
    // let tar_location: String = build_dir + package_data.name + ".tar.gz";
    
    // call download_file and pass tar_location and path
    // check errors

    // execute tar -xf <tar_location> -C <build_dir>
    // check errors

    // Check for dependencies

    // makepkg -sri
    
    let build_dir = concat_string!(RAY_TMP, "builds");

    match create_directory(build_dir) {
        Ok(()) => {

        },
        Err(_) => {
            dbg!("Error creating file");
        }
    }

    let package_name = package_data.name.unwrap();
    let tar_location = format!("{}{}{}", RAY_TMP, package_name, ".tar.gz");

    let package_url = package_data.url.unwrap();
    let download_url = format!("{}{}", BASE_URL, package_url);

    download_file(tar_location, download_url);
   

}

fn create_directory(filepath: String) ->std::io::Result<()>{

    if fs::metadata(&filepath).is_ok() {
        Ok(())
    }
    else {
        let directory = fs::File::create(filepath)?;
        let metadata  = directory.metadata()?;
        let mut permissions = metadata.permissions();

        permissions.set_mode(0o755);
        Ok(())
    }

}
