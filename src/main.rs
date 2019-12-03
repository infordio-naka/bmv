use std::path::Path;
use std::io::Write;
use std::fs::rename;

#[allow(dead_code)]
fn get_type<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn usage() {
    writeln!(std::io::stderr(), "Usage: bmv <from_prefix> <to_prefix>").unwrap();
    writeln!(std::io::stderr(),
             "example:\n\t \
              bmw /path/from_prefix /path/to_prefix"
    ).unwrap();
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len()<3 {
        usage();
        std::process::exit(1);
    }
    args[1] = String::from(args[1].split('.').nth(0).unwrap());
    let from_path = Path::new(&args[1]);
    let to_path   = Path::new(&args[2]);
    println!("from_path: {:?}", from_path);
    println!("to_path  : {:?}", to_path);
    let prefix = from_path.file_name().expect("Invalid arguments").to_str().unwrap();
    let mut parent = from_path.parent().expect("Invalid arguments");
    // println!("prefix: {:?}", prefix);
    // println!("parent: {:?}", parent);
    if parent.as_os_str().is_empty() {
        parent = Path::new("./");
    }
    for entry in parent.read_dir().expect("Directory not found") {
        if let Ok(entry) = entry {
            let entry = entry.path();
            if entry.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(prefix)
            {
                let to_name = entry.to_str().unwrap().replace(prefix, to_path.file_name().expect("Invalid arguments").to_str().unwrap());
                rename(&entry, &to_name).expect("Rename failed");
                println!("{} => {}", entry.display(), &to_name);
            }
        }
    }
}
