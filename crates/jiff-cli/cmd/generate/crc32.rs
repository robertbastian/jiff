/*!
A command for generating data tables for computing CRC32 checksums.

I took this code from the `fst` crate's build script and moved it into an
explicit generation step for Jiff. This avoids needing to build a build script
every time Jiff is compiled. There's just no real reason for it when the data
is invariant.

Jiff uses checksums to avoid two time zones with the same IANA name from
comparing equal if they are actually different. While this might sound
pathological, it can really happen if someone has loaded different versions
of tzdb into Jiff, which is a supported use case.
*/

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use lexopt::{Arg, Parser};

use crate::args::{self, Usage};

const USAGE: &'static str = r#"
Generate Rust source code for CRC32 data tables.

USAGE:
    jiff-cli generate crc32 [<jiff-dir>]

This command generates the requisite Rust source code for CRC32 data tables.
Specifically for the Castagnoli polynomial.

Jiff uses checksums for facilitating reasonably accurate (albeit not perfect)
time zone equality comparisons.

This program should be run from the root of the Jiff repository. Alternatively,
provide a path to the root of the Jiff repository as a positional argument.
"#;

const CASTAGNOLI_POLY: u32 = 0x82f63b78;

pub fn run(p: &mut Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    args::configure(p, USAGE, &mut [&mut config])?;

    let jiff = config.jiff();
    let table_path = jiff.join("src/shared/crc32/table.rs");
    write_crc_tables(&table_path).with_context(|| {
        format!("failed to write CRC32 data table to {}", table_path.display())
    })?;
    super::rustfmt(&table_path)?;

    Ok(())
}

#[derive(Debug)]
struct Config {
    jiff: Option<PathBuf>,
    verbose: bool,
}

impl Config {
    fn jiff(&self) -> &Path {
        self.jiff.as_deref().unwrap_or_else(|| Path::new("./"))
    }
}

impl Default for Config {
    fn default() -> Config {
        Config { jiff: None, verbose: false }
    }
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        _: &mut Parser,
        arg: &mut Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            Arg::Short('v') | Arg::Long("verbose") => {
                self.verbose = true;
            }
            Arg::Value(ref mut value) => {
                if self.jiff.is_none() {
                    let path = PathBuf::from(std::mem::take(value));
                    self.jiff = Some(path);
                } else {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        const USAGES: &'static [Usage] = &[Usage::new(
            "-v, --verbose",
            "Add more output.",
            r#"
This is a generic flag that expands output beyond the "normal" amount. Which
output is added depends on the command.
"#,
        )];
        USAGES
    }
}

fn write_crc_tables(path: &Path) -> anyhow::Result<()> {
    let mut out = BufWriter::new(File::create(path)?);

    let table = make_table(CASTAGNOLI_POLY);
    let table16 = make_table16(CASTAGNOLI_POLY);

    writeln!(out, "// auto-generated by: jiff-cli generate crc32")?;
    writeln!(out, "")?;
    writeln!(out, "pub(super) const TABLE: [u32; 256] = [")?;
    for &x in table.iter() {
        writeln!(out, "    {},", x)?;
    }
    writeln!(out, "];\n")?;

    writeln!(out, "pub(super) const TABLE16: [[u32; 256]; 16] = [")?;
    for table in table16.iter() {
        writeln!(out, "    [")?;
        for &x in table.iter() {
            writeln!(out, "        {},", x)?;
        }
        writeln!(out, "    ],")?;
    }
    writeln!(out, "];")?;

    out.flush()?;

    Ok(())
}

fn make_table16(poly: u32) -> [[u32; 256]; 16] {
    let mut tab = [[0; 256]; 16];
    tab[0] = make_table(poly);
    for i in 0..256 {
        let mut crc = tab[0][i];
        for j in 1..16 {
            crc = (crc >> 8) ^ tab[0][usize::from(crc as u8)];
            tab[j][i] = crc;
        }
    }
    tab
}

fn make_table(poly: u32) -> [u32; 256] {
    let mut tab = [0; 256];
    for i in 0u32..256u32 {
        let mut crc = i;
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ poly;
            } else {
                crc >>= 1;
            }
        }
        tab[usize::try_from(i).unwrap()] = crc;
    }
    tab
}
