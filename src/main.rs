use std::{
    collections::{HashMap, BTreeMap},
    fs::{self},
    path::Path,
    str::FromStr, fmt::Debug,
};

use serde_json::json;

fn main() {
    let path = Path::new("src/common/responses/");
    let path2 = Path::new("src/C9977_SampleTest/responses/");
    let mut result = BTreeMap::new();
    // take_file_paths(path, &mut path1Files);
    // take_file_paths(path2, &mut path2Files);
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

        // for n in 0..if path1Files.len() > path2Files.len() {
        //     path1Files.len()
        // } else {
        //     path2Files.len()
        // } {
        //     println!(
        //         "{:?}: {:?} and {:?} are equals: {:?}","comparing files",path1Files.get(n).unwrap(),path2Files.get(n).unwrap(),
        //         json!(fs::read_to_string(path1Files.get(n).unwrap()).unwrap())
        //             == json!(fs::read_to_string(path2Files.get(n).unwrap()).unwrap())
        //     );
        // }
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
