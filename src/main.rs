use std::fs::{read_dir, rename};
use std::io::stdin;
use std::vec::Vec;
use std::path::Path;

//자릿수를 입력받습니다.
fn read_cipher() ->u8
{
    const ERROR_LOG: &str = "잘못된 입력입니다. 1-255 범위의 자연수만 입력 가능합니다.";

    let cipher:u8 =
        {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).expect("읽기 실패");
            buffer.trim().parse::<u8>().expect(ERROR_LOG)
        };

    if cipher == 0
    {
        panic!(ERROR_LOG);
    }

    return cipher;
}

//디렉터리 파일들을 읽어옵니다.
fn read_files() ->Vec<String>
{
    let mut filenames = Vec::<String>::new();

    for _entry in read_dir("").expect("디렉터리 읽기 실패")
    {
        let entry = _entry.unwrap();

        let is_file = entry.file_type().unwrap().is_file();

        if is_file
        {
            filenames.push(entry.file_name().to_str().unwrap().to_string());
        }
    }

    return filenames;
}

//이름을 처리합니다.
fn rename_files(files: Vec<String>, cipher:u8)
{
    let mut i = 0;

    'OUTER_LOOP:
    for filename in files
    {
        let save_filename = filename.clone();

        //확장자 획득
        let extension =
        {
            let wrapped_extension = Path::new(filename.as_str()).extension();
            if wrapped_extension.is_some()
            {
                ".".to_string()+wrapped_extension.unwrap().to_str().unwrap()
            }
            else
            {
                "".to_string()
            }
        };

        //확장자 필터링
        let no_change_list = vec![".exe", ".out"];
        for e in no_change_list
        {
            if e == extension.as_str()
            {
                continue 'OUTER_LOOP;
            }
        }

        let new_name = format!("{:0cipher$}", i, cipher=cipher as usize);

        rename(save_filename, new_name+extension.as_str()).expect("이름 변경 실패");

        i+=1;
    }
}

fn main()
{
    println!("## 파일 읽기 시작 ##");
    let filenames = read_files();
    println!("! 넘버링할 숫자의 자릿수를 입력해주세요.(1-255)");
    let cipher = read_cipher();
    println!("\n## 처리 시작 ##");
    rename_files(filenames, cipher);
    std::process::Command::new("pause");
}