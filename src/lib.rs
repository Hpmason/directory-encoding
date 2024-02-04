use std::{
    error::Error,
    fs::{self, File},
    io::{Read, Write},
};

use cli::{Cli, Mode};

pub mod cli;

pub fn encode(args: Cli) -> Result<(), Box<dyn Error>> {
    let Mode::Encode { file, directory } = args.mode else {
        return Err("Called encode with Mode::Decode args")?;
    };
    if !file.exists() {
        Err(format!("{} does not exist", file.to_string_lossy()))?;
    }
    if !file.is_file() {
        Err(format!("{} is not a file", file.to_string_lossy()))?;
    }

    let mut f = File::open(file)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    if directory.exists() {
        if !args.force {
            Err(format!(
                "directory to encode to, {}, already exists\n\
                Delete the directory if you want to use this as the encoding directory",
                directory.to_string_lossy()
            ))?;
        }
        fs::remove_dir_all(&directory)?;
    }
    fs::create_dir_all(&directory)?;
    for (i, byte) in buf.into_iter().enumerate() {
        let dir = directory
            .join(format!("{i:016X}"))
            .join(format!("{byte:02X}"));
        fs::create_dir_all(dir)?;
    }

    Ok(())
}

pub fn decode(args: Cli) -> Result<(), Box<dyn Error>> {
    let Mode::Decode { directory, file } = args.mode else {
        return Err("Called decode with Mode::Encode args")?;
    };
    if !directory.exists() {
        Err(format!("{} does not exist", directory.to_string_lossy()))?;
    }
    if !directory.is_dir() {
        Err(format!(
            "{} is not a directory",
            directory.to_string_lossy()
        ))?;
    }

    if !args.force && file.exists() {
        Err(format!(
            "File {} already exists, please use a different name for decoded file",
            file.to_string_lossy()
        ))?;
    }
    let mut buf = Vec::new();
    for dir in fs::read_dir(directory)? {
        let Ok(entry) = dir else {
            eprintln!("Could not get directory entry");
            continue;
        };
        let Ok(ty) = entry.file_type() else {
            eprintln!("Could not get entry type");
            continue;
        };
        if !ty.is_dir() {
            eprintln!("Found file, invalid encoding type");
            continue;
        }
        let offset = u64::from_str_radix(&entry.path().file_name().unwrap().to_string_lossy(), 16)
            .map_err(|_| format!("Could not parse offset directory name"))?;
        let byte = u8::from_str_radix(&entry.path().read_dir()?.next().unwrap().unwrap().file_name().to_string_lossy(), 16)
            .map_err(|_| format!("Could not parse byte directory name"))?;
        if buf.len() <= offset as usize {
            let growth = offset as usize - buf.len() + 1;
            for _ in 0..growth {
                buf.push(0);
            }
        }
        buf[offset as usize] = byte;
    }
    let mut f = File::create(file)?;
    f.write_all(&buf)?;

    Ok(())
}
