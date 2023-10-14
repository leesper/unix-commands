use std::{io::Read, io::Cursor, env, fs::File};

const PAGE_LEN: i32 = 24;
const LINE_LEN: i32 = 512;
fn main() {
    let args = env::args().skip(1).collect();
    let file_list = filter_files(args);
    for file in &file_list {
        println!("{}", file);
    }
}

fn filter_files(args: Vec<String>) -> Vec<String> {
    let mut file_list: Vec<String> = Vec::new();
    for arg in args.iter() {
        if !arg.starts_with("-") {
            file_list.push(arg.to_string());
        }
    }
    return file_list;
}

fn see_more<R>(mut reader: R) -> i32 where R: Read {
    let mut cmd = [0; 1];
    let mut nbytes = reader.read(&mut cmd).unwrap();
    let mut reply: i32 = -1;

    while nbytes > 0 {
        reply = match cmd[0] as char {
            ' ' => PAGE_LEN,
            '\n' => 1,
            'q' => 0,
            _ => continue
        };
        nbytes = reader.read(&mut cmd).unwrap();
    }

    return reply;
}
// requirements: a more command in rust
// 1. more filename...
// (1) show the contents of files one by one
// (2) show the content of one file page by page(24 lines * 512 characters)
// (3) hit SPACE to advance next page
// (4) hit ENTER to advance next one line
// (5) hit q to exit
// 2. command | more
// (1) pipe another command's output as input
// 3. more < filename
// (1) redirect file content as more's input

// function points:
// 1. more file
// (1) args -> file list
// should filter out files from args
// (2) display page
// TODO: should return error if file not exist
// TODO: should read at most 512 characters per line
// (3) process user command
// TODO: should read next page of 24 lines if ENTER
// TODO: should read next line if SPACE
// TODO: should read no line if q

// let filenames = get file list from args;
// for name in filenames {
//      let file = File::new(name);
//      do_more(&file);
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_filter_out_files_from_args() {
        let args = vec!["-h".to_string(), "file1".to_string(), "-s".to_string(), "-t".to_string(), "file2".to_string(), "file3".to_string()];
        let file_list = filter_files(args);
        assert_eq!(vec!["file1".to_string(), "file2".to_string(), "file3".to_string()], file_list);
    }

    // - more
    #[test]
    fn should_return_error_if_file_not_exist() {
        let result = File::open("file_not_exist");
        assert!(result.is_err());
    }
    #[test]
    fn should_see_more_behave_as_expected() {
        let cases = vec![
            (" ", PAGE_LEN, format!("{} expected", PAGE_LEN)),
            ("\n", 1, "1 expected".to_string()),
            ("q", 0, "0 expected".to_string()),
        ];
        for (input, expected, message) in cases {
            let reader = Cursor::new(input);
            let actual = see_more(reader);
            assert_eq!(expected, actual, "{}", message);
        }
    }
}



