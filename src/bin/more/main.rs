use std::env;

fn main() {
    let args = env::args().collect();
    let file_list = parse_file_names(args);
    for file in &file_list {
        println!("{}", file);
    }
}

fn parse_file_names(args: Vec<String>) -> Vec<String> {
    let mut file_list: Vec<String> = Vec::new();
    for arg in args.iter() {
        if !arg.starts_with("-") {
            file_list.push(arg.to_string());
        }
    }
    return file_list;
}

// 1. more filename...
// let filenames = get file list from args;
// let morer = new Morer();
// for name in filenames {
//      let file = File::new(name);
//      morer.do_more(&file);
// }
// - Args
#[test]
fn should_retrieve_file_list_from_args() {
    let args = vec!["-h".to_string(), "file1".to_string(), "-s".to_string(), "-t".to_string(), "file2".to_string(), "file3".to_string()];
    let file_list = parse_file_names(args);
    assert_eq!(vec!["file1".to_string(), "file2".to_string(), "file3".to_string()], file_list);
}


// - Morer
// TODO: should display a screenful of text in terminal(24*512)
// TODO: should display a ":" at the bottom
// TODO: should read user input from keyboard
// TODO: should advance one screen if SPACE
// TODO: should advance one line if ENTER
// TODO: should exit if q
// 2. command | more
// 3. more < filename