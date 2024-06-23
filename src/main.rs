use std::borrow::Cow;

use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use std::path::Path;
use std::path::PathBuf;

use std::process;

use regex::bytes::Regex;

fn main() {

    let path_arg = std::env::args().nth(1).expect("no pdf path given");
    
    let replace_flag:bool = match std::env::args().nth(2) {
        Some(arg) => {
            if arg == "replace".to_string() {
                true
            } else {
                panic!("no surch arg as {}", arg);
               
            }
        }
        None => false,
    };

    let path = PathBuf::from(path_arg);
    if !path.exists() {
        panic!("file does not exist.");
    }

    if path.extension().unwrap() != "pdf" {
        panic!("file is not a pdf file");
    }
    

    let pdf_file = std::fs::File::open(&path).unwrap();
    let mut buffer = BufReader::new(pdf_file);
    let mut content = Vec::new();
    buffer.read_to_end(&mut content).unwrap();
    
    let new_content = replace_fit_bookmark(&content);

    if replace_flag {
        save_file(&path, &new_content).expect("error replacing pdf file");
    } else {
        let mut file_name = path.file_stem().unwrap().to_os_string();
        file_name.push("_(new)");
        let path_dest = change_file_name(&path, file_name.to_str().unwrap());
        save_file(&path_dest, &new_content).expect("error saving new pdf file");
    }
    
    println!("Document transformed.");
    process::exit(0);

}

fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}

fn replace_fit_bookmark(haystack: &[u8]) -> Cow<[u8]> {
    let to_bytes = "/XYZ null null null".to_string().into_bytes();
    let re = Regex::new(r"/Fit").unwrap();
    let new_content = re.replace_all(haystack, &to_bytes);
    match new_content {
        Cow::Borrowed(_haystack) => {
            println!("No /Fit Page Bookmark destination found in this document");
            println!("Exit without changing the document");
            std::process::exit(0)
        },

        _ => new_content,
    }
    
}

fn save_file(path: &PathBuf, content: &[u8]) -> std::io::Result<()> {
    let mut new_file = std::fs::File::create(path)?;
    new_file.write_all(content)?;
    Ok(())
}
// ten times slower than regexp replace
fn _replace<T>(source: &[T], from: &[T], to: &[T]) -> Vec<T>
where
    T: Clone + PartialEq
{
    let mut result = source.to_vec();
    let from_len = from.len();
    let to_len = to.len();

    let mut i = 0;
    while i + from_len <= result.len() {
        if result[i..].starts_with(from) {
            result.splice(i..i + from_len, to.iter().cloned());
            i += to_len;
        } else {
            i += 1;
        }
    }

    result
}

