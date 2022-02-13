use rand::seq::SliceRandom;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;

pub enum TestFile {
    Random,
    AllTheBytes,
    NoPermissions,
}

impl TestFile {
    const _RANDOM: &'static str = "test_inputs/random.txt";
    const _ALL_THE_BYTES: &'static str = "test_inputs/all_the_bytes.txt";
    const _NO_PERMISSIONS: &'static str = "test_inputs/no_permision.txt";

    fn get_path(&self) -> PathBuf {
        match self {
            TestFile::Random => PathBuf::from(TestFile::_RANDOM),
            TestFile::AllTheBytes => PathBuf::from(TestFile::_ALL_THE_BYTES),
            TestFile::NoPermissions => PathBuf::from(TestFile::_NO_PERMISSIONS),
        }
    }
    fn create(&self) {
        let file = File::create(self.get_path()).unwrap();
        let mut writer = BufWriter::new(file);
        match self {
            TestFile::Random => {
                let mut chars: Vec<u8> = (33u8..=0x7E).collect::<Vec<u8>>();
                chars.append(&mut vec![9, 10]);
                let mut rng = rand::thread_rng();
                for _ in 0..=10_000_000 {
                    write!(writer, "{}", *chars.choose(&mut rng).unwrap() as char).unwrap();
                }
            }
            TestFile::AllTheBytes => {
                for byte in 0..=255u8 {
                    write!(writer, "{} = ", byte).unwrap();
                    writer.write_all(&[byte][..]).unwrap();
                    writeln!(writer).unwrap();
                }
            }
            TestFile::NoPermissions => {
                let path = self.get_path();
                Command::new("chmod")
                    .args(["000", path.to_str().unwrap()])
                    .spawn()
                    .expect("Fail creating child")
                    .wait()
                    .expect("Fail changing permission");
            }
        }
    }
    pub fn get(&self) -> &'static str {
        let path = self.get_path();
        if !path.exists() {
            self.create()
        }
        match self {
            TestFile::Random => TestFile::_RANDOM,
            TestFile::AllTheBytes => TestFile::_ALL_THE_BYTES,
            TestFile::NoPermissions => TestFile::_NO_PERMISSIONS,
        }
    }
}

impl AsRef<OsStr> for TestFile {
    fn as_ref(&self) -> &OsStr {
        &OsStr::new(self.get())
    }
}
