use std::fmt;
use std::fs::File;
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use crate::error::Result;

pub struct Storage {
    file: File,
}

impl Storage {
    const METADATA_RESERVED_SIZE: usize = 1024;
    const SC_OFFSET: usize = 0;
    const SC_SIZE: usize = 32;
    const LUT_OFFSET: usize = Self::SC_OFFSET + Self::SC_SIZE;
    const LUT_SIZE: usize = 512;
    const RESERVED_LATER_USE_OFFSET: usize = Self::LUT_OFFSET + Self::LUT_SIZE;
    const RESERVED_LATER_USE_SIZE: usize = Self::METADATA_RESERVED_SIZE - Self::RESERVED_LATER_USE_OFFSET;

    pub fn new() -> Result<Self>
    {
        // if debug, else nominal -- but this I don't like this thing.
        let path = if cfg!(debug_assertions) {
            PathBuf::from(".todo_storage")
        } else {
            let home = std::env::var("HOME")
                .or(Err("HOME is not set, cannot find default storage file!"))?;
            PathBuf::from(home).join(".todo_storage")
        };

        let mut options = std::fs::OpenOptions::new();
        options.read(true);

        let file = match path.exists() {
            true => options.append(true),
            false => options.write(true).create_new(true)
        }.open(&path)?;

        if path.exists() {
            return Ok(Self { file });
        }

        const SC_BYTES: &[u8; Storage::SC_SIZE] = b"_This_Is_An_Epic_Sanity_Check_V1"; // checked at compile time, nice!
        file.write_all_at(SC_BYTES, Storage::SC_OFFSET as u64)?;

        const LUT_BYTES: &[u8; Storage::LUT_SIZE] = &[0; 512];
        file.write_all_at(LUT_BYTES, Storage::LUT_OFFSET as u64)?;

        const RESERVED_BYTES: &[u8; Storage::RESERVED_LATER_USE_SIZE] = &[0; Storage::RESERVED_LATER_USE_SIZE];
        file.write_all_at(RESERVED_BYTES, Storage::RESERVED_LATER_USE_OFFSET as u64)?;
        debug_assert_eq!(Storage::METADATA_RESERVED_SIZE as u64, file.metadata().unwrap().len());

        Ok(Self { file })
    }
    pub fn from_path(p: &Path) -> Result<Self> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .open(&p)?;
        Ok(Self { file })
    }
}

// custom impl of fmt::Debug
impl fmt::Debug for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buff = [0u8; Storage::METADATA_RESERVED_SIZE];
        self.file.read_at(&mut buff, 0).unwrap();

        let sanitize = |slice: &[u8]| -> String {
            slice.iter()
                .map(|&c| if c == 0 { '.' } else { c as char })
                .collect()
        };

        write!(f, "Storage [
    Sanity Check: {},
    Look Up Table: {},
    Reserved for Later Use: {},
]",
               sanitize(&buff[Storage::SC_OFFSET..Storage::SC_SIZE]),
               sanitize(&buff[Storage::LUT_OFFSET..Storage::LUT_SIZE]),
               sanitize(&buff[Storage::RESERVED_LATER_USE_OFFSET..Storage::METADATA_RESERVED_SIZE]))
    }
}
