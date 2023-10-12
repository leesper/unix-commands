use std::{io, env, fs::File, str::FromStr};
use std::io::Read;

const PAGE_LEN: u32 = 24;
const LINE_LEN: u32 = 512;
fn main() {
    let args = env::args().skip(1).collect();
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

fn see_more<R>(mut reader: R) -> u32 where R: Read {
    let mut cmd = [0; 1];
    let _ = reader.read(&mut cmd);
    let mut reply: u32 = 0;

    if cmd[0] as char == ' ' {
        reply = PAGE_LEN;
    }

    return reply;
}

// 1. more filename...
// let filenames = get file list from args;
// for name in filenames {
//      let file = File::new(name);
//      do_more(&file);
// }
// 2. command | more
// 3. more < filename
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn should_retrieve_file_list_from_args() {
        let args = vec!["-h".to_string(), "file1".to_string(), "-s".to_string(), "-t".to_string(), "file2".to_string(), "file3".to_string()];
        let file_list = parse_file_names(args);
        assert_eq!(vec!["file1".to_string(), "file2".to_string(), "file3".to_string()], file_list);
    }

    // - more
    #[test]
    fn should_return_error_if_file_not_exist() {
        let result = File::open("file_not_exist");
        assert!(result.is_err());
    }
    #[test]
    fn should_see_more_one_page_if_space() {
        let repeater = io::repeat(u8::try_from(' ').unwrap());
        let answer = see_more(repeater);
        assert_eq!(PAGE_LEN, answer);
    }
    // TODO: should see_more() return 1 if ENTER
    // TODO: should see_more() return 0 if q
}



