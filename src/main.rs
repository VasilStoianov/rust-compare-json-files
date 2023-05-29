use std::{
    collections::{BTreeMap},
    env,
    fs::{self},
    path::Path,
    str::FromStr
};

use serde_json::json;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",&args);
    if args.len() > 2 {
        let path = Path::new(&args[1]);
        let path2 = Path::new(&args[2]);
        compare_files(path, path2);
    }
    else {
        let path = Path::new(&args[1]);
        compare_files(path, path);
    }
   

}

fn compare_files(path: &Path, path2: &Path){
    let mut result = BTreeMap::new();
    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        let pathfile1 = match entry.path().to_str() {
            Some(path) => path.to_string(),
            None => String::from_str("no file").unwrap(),
        };
        for second_entry in fs::read_dir(path2).expect("No files in second dir or unable to find") {
            let second_entry = second_entry.expect("unable to get files");
            let pathfile2 = match second_entry.path().to_str() {
                Some(path) => path.to_string(),
                None => String::from_str("no file").unwrap(),
            };
            let are_equals = json!(fs::read_to_string(&pathfile1).unwrap())
                == json!(fs::read_to_string(&pathfile2).unwrap());
            result.insert(
                format!("Comparing : {} and {} ", pathfile1, String::from(pathfile2)),
                are_equals,
            );
            if are_equals {
                break;
            }
        }

    }
    
    for line in result.keys() {
        println!("{:?} {:?}",line,result.get(line).unwrap() );
    }
}

fn take_file_paths(path: &Path, vec: &mut Vec<String>) {
    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        vec.push(match entry.path().to_str() {
            Some(path) => path.to_string(),
            None => String::from_str("no file").unwrap(),
        });
    }
}
