mod aur;
mod args; 

use colored::*;
use std::io;
use std::io::Write;
use std::env;

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
    //print_package_data(&package_data);
    aur::install(&package_data[0]).await;

}

fn print_package_data(package_data: &Vec<aur::PackageData>) {
    for package in package_data {
        if package.name == None {
            println!("Name: None");
        } else {
            println!("{}: {}", "Package".bright_green(), package.name.as_ref().unwrap());
        }

        if package.version == None {
            println!("Version: None");
        } else {
            println!(
                "{}: {}",
                "Version".bright_magenta(),
                package.version.as_ref().unwrap()
            );
        }
        if package.description == None {
            println!("Description: None");
        } else {
            println!(
                "{}: {}",
                "Description".bright_cyan(),
                package.description.as_ref().unwrap()
            );
        }

        if package.maintainer == None {
            println!("Maintainer: None");
        } else {
            println!("Maintainer: {}", package.maintainer.as_ref().unwrap());
        }
        println!();
    }
}
