use std::path::Path;
use std::{env, error::Error};

use ironworks::{
    Ironworks,
    excel::{Excel, Language},
    sqpack::{Install, SqPack},
};
use regex::Regex;

mod exd_schema;
mod export;
mod formatter;

// Taiwan client uses language code 8 (ChineseTraditionalTW)
const LANGUAGES: [Language; 1] = [
    Language::ChineseTraditionalTW,
];

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let ironworks = Ironworks::new().with_resource(SqPack::new(Install::at(path)));
    let mut excel = Excel::new(ironworks);

    // Skip sheets without schemas (quest/, custom/, etc.)
    let skip_sheet_regex = Regex::new(r"\/").unwrap();

    for language in LANGUAGES {
        excel.set_default_language(language);

        let mut success_count = 0;
        let mut skip_count = 0;

        for sheet in excel.list().unwrap().iter() {
            if skip_sheet_regex.is_match(&sheet) {
                continue;
            }

            match export::sheet(&excel, language, &sheet) {
                Ok(_) => {
                    success_count += 1;
                    if success_count % 100 == 0 {
                        println!("Exported {} sheets...", success_count);
                    }
                },
                Err(e) => {
                    // Skip sheets that don't have language-specific data
                    skip_count += 1;
                    if skip_count <= 10 {
                        eprintln!("Skipping {}: {}", sheet, e);
                    }
                }
            }
        }

        println!("Completed: {} sheets exported, {} skipped", success_count, skip_count);
    }

    Ok(())
}
