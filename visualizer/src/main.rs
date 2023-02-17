use std::ffi::OsStr;
use std::fs::DirEntry;

use visualizer::visualization;

fn main() -> anyhow::Result<()> {
    // 1. 首先先遍历一遍temp文件夹找出还没有生成相同文件名的png文件的csv文件
    let mut png_list: Vec<DirEntry> = vec![];
    let mut csv_list: Vec<DirEntry> = vec![];
    for dir_entry in std::fs::read_dir("./temp").unwrap() {
        let dir_entry = dir_entry?;
        if dir_entry.file_type().unwrap().is_file() {
            if dir_entry.path().extension() == Some(OsStr::new("png")) {
                png_list.push(dir_entry);
            } else if dir_entry.path().extension() == Some(OsStr::new("csv")) {
                csv_list.push(dir_entry);
            }
        }
    }
    let mut process_list: Vec<(String, &DirEntry)> = vec![];
    for csv in csv_list.as_slice() {
        let filename = csv.file_name();
        let captures = regex::Regex::new(r"(.*).csv")
            .unwrap()
            .captures(filename.to_str().unwrap())
            .unwrap();
        let csv_name = captures[1].parse::<String>().unwrap();
        let mut if_exist = false;
        for png in png_list.as_slice() {
            let filename = png.file_name();
            let captures = regex::Regex::new(r"(.*).png")
                .unwrap()
                .captures(filename.to_str().unwrap())
                .unwrap();
            let png_name = captures[1].parse::<String>().unwrap();
            if csv_name == png_name {
                if_exist = true;
            }
        }
        if !if_exist {
            process_list.push((csv_name, csv));
        }
    }
    // 2. 然后每个csv文件生成对应png文件
    for (name, file) in process_list {
        visualization(name, file.path().to_str().unwrap())?;
    }
    Ok(())
}
