mod aur;
mod args; 

use colored::*;
use std::io;
use std::io::Write;

#[macro_use(concat_string)]
extern crate concat_string;

#[tokio::main]
async fn main() {
    print!("Enter package name >> ");
    io::stdout().flush().unwrap();

    let mut package = String::new();
    io::stdin()
        .read_line(&mut package)
        .expect("Cannot read line");

    let package: String = package
        .trim()
        .parse()
        .expect("Cannot parse the package name");

    let package_data = match aur::search_aur(package).await {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };

    // args::arg_parser();

    println!();
    print_package_data(&package_data);
    
    let packages_to_install = get_package_index(package_data);

    aur::install(packages_to_install).await;

}

fn print_package_data(package_data: &Vec<aur::PackageData>) {
    let mut idx: u32 = 0;
    for package in package_data {
        if package.name == None {
            println!("Name: None");
        } else {
            println!("{}: {}", idx, package.name.as_ref().unwrap());
        }

        idx+=1;

        // if package.version == None {
        //     println!("Version: None");
        // } else {
        //     println!(
        //         "{}: {}",
        //         "Version".bright_magenta(),
        //         package.version.as_ref().unwrap()
        //     );
        // }
        // if package.description == None {
        //     println!("Description: None");
        // } else {
        //     println!(
        //         "{}: {}",
        //         "Description".bright_cyan(),
        //         package.description.as_ref().unwrap()
        //     );
        // }

        // if package.maintainer == None {
        //     println!("Maintainer: None");
        // } else {
        //     println!("Maintainer: {}", package.maintainer.as_ref().unwrap());
        // }
        //println!();
    }
}

fn get_package_index(package_data: Vec<aur::PackageData>) -> Vec<aur::PackageData> {

    println!("Enter packages to install with space");

    let mut packages = String::new();
    io::stdin()
        .read_line(&mut packages)
        .expect("Cannot read line");

    let package_indexes: Vec<u32> = packages.trim()
                          .split_whitespace()
                          .map(|s| s.parse().unwrap())
                          .collect();

    let mut packages_to_install: Vec<aur::PackageData> = Vec::new();
    for i in package_indexes {
        let package = &package_data[i as usize];
        packages_to_install.push(package.clone());
    }

    packages_to_install
}
