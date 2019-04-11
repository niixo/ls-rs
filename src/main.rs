use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;
use std::io::{BufWriter, stdout, Write};

fn main() {
    let mut dir: Vec<_> = fs::read_dir(".").unwrap().map(|x| x.unwrap()).collect();
    dir.sort_by_key(|x| x.path());

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    for entry in dir {
        let meta = entry.metadata().unwrap();
        let file_type = meta.file_type();
        let mode = meta.permissions().mode();

        let output = if file_type.is_symlink() {
            "l"
        } else if meta.is_dir() {
            "d"
        } else {
            "-"
        };

        let ref_mode = format!("{:o}", mode).chars().rev().take(3).collect::<String>();
        let mode = ref_mode.chars().rev().collect::<String>();

        // 120 symlink
        // 100 file
        // 40 directory
        writeln!(out, "{}{} {:>7} {}",
                 output,
                 format_permissions(mode),
                 fs::metadata(entry.path()).unwrap().size(),
                 entry.file_name().to_str().unwrap()
        ).unwrap();
    }
}

fn format_permissions(permissions: String) -> String {
    permissions.chars().map(|x| {
        match x {
            '7' => "rwx",
            '6' => "rw-",
            '5' => "r-x",
            '4' => "r--",
            '3' => "-wx",
            '2' => "-w-",
            '1' => "--x",
            _ => "---"
        }
    }).collect::<String>()
}