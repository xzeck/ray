use std::env;
use std::fmt;

#[derive(Debug)]
struct search_args {
    name: bool,
    name_desc: bool,
    maintainer: bool,
    depends: bool,
    make_depends: bool,
    opt_depends: bool,
    check_depends: bool
}


pub fn arg_parser() {

    let args: Vec<String> = env::args().collect();
    
    let mut search_args_params = search_args {
        name: false, 
        name_desc: false,
        maintainer: false,
        depends: false,
        make_depends: false,
        opt_depends: false,
        check_depends: false,
    };
    

    println!("{:?}", args);

    for arg in args {
        match arg.as_str() { 
            "n"=> search_args_params.name = true,
            "nd" => search_args_params.name_desc = true,
            "m" => search_args_params.maintainer = true,
            "d" => search_args_params.depends = true,
            "md" => search_args_params.make_depends = true,
            "od" => search_args_params.opt_depends = true,
            "cd" => search_args_params.check_depends = true,
            _ => {
                break;
            }

        }
    }

    println!("{:?}", search_args_params);
}

fn query_builder(search_params: search_args) {
    
}
