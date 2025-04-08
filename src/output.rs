use std::fs::File;
use std::io::{self, BufWriter, Write, stdout};

// Assuming final_variations is a Vec<String> or HashSet<String>
pub(crate) fn output_results(final_variations: impl IntoIterator<Item = String>, out_file: Option<String>) -> io::Result<()> {
    match out_file {
        Some(file_path) => {
            let file = File::create(file_path)?;
            let mut writer = BufWriter::new(file);

            for variant in final_variations {
                writeln!(writer, "{}", variant)?;
            }
        }
        None => {
            let stdout = stdout();
            let mut writer = stdout.lock();

            for variant in final_variations {
                writeln!(writer, "{}", variant)?;
            }
        }
    }

    Ok(())
}