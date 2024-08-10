use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    ops::{Deref, DerefMut},
};

use clap::{Parser, Subcommand};
use prometheus::fs::read_lines;

const ENV_FILE_HEADERS: &str =
    "| Key | Responsible | Type | Secret | Policy | Default value | Description | Docs |";

const ENV_FILE_HEADER_SEPARATOR: &str = "| --------- | --------- | --------- | --------- | --------- | --------- | --------- | --------- |";

#[derive(Debug, Parser)]
#[command(name = "milu")]
#[command(about = "A CLI tool for Milú Frontend convenience utilities", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generates .env file documentation according to Milú specification
    #[command(arg_required_else_help = true)]
    GenerateEnvDocs {
        /// The .env file to generate documentation from
        file: String,
    },
}

#[derive(Debug)]
struct Line {
    data: String,
    kind: LineKind,
}

impl Line {
    pub fn new(data: String) -> Self {
        let trimmed_data = data.trim();

        let kind = match trimmed_data {
            _ if trimmed_data.starts_with(LineKind::Responsible.get_prefix()) => {
                LineKind::Responsible
            }
            _ if trimmed_data.starts_with(LineKind::Type.get_prefix()) => LineKind::Type,
            _ if trimmed_data.starts_with(LineKind::Secret.get_prefix()) => LineKind::Secret,
            _ if trimmed_data.starts_with(LineKind::Policy.get_prefix()) => LineKind::Policy,
            _ if trimmed_data.starts_with(LineKind::Docs.get_prefix()) => LineKind::Docs,
            _ if trimmed_data.starts_with(LineKind::Description.get_prefix()) => {
                LineKind::Description
            }
            _ => LineKind::EnvVariable,
        };

        Self { data, kind }
    }

    pub fn extract_content(&self) -> String {
        self.data
            .strip_prefix(self.kind.get_prefix())
            .unwrap_or(&self.data)
            .strip_suffix(self.kind.get_suffix())
            .unwrap_or(&self.data)
            .to_string()
    }
}

#[derive(Debug)]
enum LineKind {
    EnvVariable,
    Responsible,
    Type,
    Secret,
    Policy,
    Description,
    Docs,
}

impl LineKind {
    fn get_prefix(&self) -> &'static str {
        match self {
            LineKind::Responsible => "# [@responsible=",
            LineKind::Type => "# [@type=",
            LineKind::Secret => "# [@secret=",
            LineKind::Policy => "# [@policy=",
            LineKind::Docs => "# [@docs=",
            LineKind::Description => "#",
            _ => "",
        }
    }

    fn get_suffix(&self) -> &'static str {
        match self {
            LineKind::Responsible
            | LineKind::Type
            | LineKind::Secret
            | LineKind::Policy
            | LineKind::Docs => "]",
            _ => "",
        }
    }
}

#[derive(Debug)]
enum FieldKind {
    EnvVariable = 0,
    Responsible = 1,
    Type = 2,
    Secret = 3,
    Policy = 4,
    DefaultValue = 5,
    Description = 6,
    Docs = 7,
}

impl From<FieldKind> for u8 {
    fn from(lk: FieldKind) -> u8 {
        lk as u8
    }
}

struct Output(HashMap<u8, String>);

impl Deref for Output {
    type Target = HashMap<u8, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Output {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Output {
    pub fn new() -> Self {
        Output(HashMap::with_capacity(8))
    }

    /// Given a a HashMap index inserts an element into the Output HashMap. If a value is already
    /// present in this index it will concatenate it using "," separator.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut output = Output::new();
    ///
    /// output.add_at(1, String::from("hello"));
    /// output.add_at(1, String::from("world"));
    ///
    /// assert_eq!(output.get(&1), Some(&"hello,world"));
    /// ```
    pub fn add_at(&mut self, index: u8, value: String) {
        self.entry(index)
            .and_modify(|stored_value| {
                stored_value.push(',');
                stored_value.push_str(value.as_str());
            })
            .or_insert(value);
    }

    pub fn as_string(&self) -> String {
        let empty_string = &String::default();

        format!(
            "|{}|{}|{}|{}|{}|{}|{}|{}|\n",
            self.get(&FieldKind::EnvVariable.into())
                .unwrap_or(empty_string),
            self.get(&FieldKind::Responsible.into())
                .unwrap_or(empty_string),
            self.get(&FieldKind::Type.into()).unwrap_or(empty_string),
            self.get(&FieldKind::Secret.into()).unwrap_or(empty_string),
            self.get(&FieldKind::Policy.into()).unwrap_or(empty_string),
            self.get(&FieldKind::DefaultValue.into())
                .unwrap_or(empty_string),
            self.get(&FieldKind::Description.into())
                .unwrap_or(empty_string),
            self.get(&FieldKind::Docs.into()).unwrap_or(empty_string),
        )
    }
}

fn main() {
    let args = Cli::parse();
    // Explanation: Command script gathers line by line until it reaches a value and then outputs all that it gathered into one table. Then it starts over.
    //
    // EXPECTED OUTPUT:
    //
    // | Key      | Responsible | Type | Secret | Policy | Default value | Description | Docs |
    // | LINE_VAL | | | | | | DOCSTRINGS | |

    match args.command {
        Commands::GenerateEnvDocs { file } => {
            let output_file = File::create("environment-variables.md")
                .expect("File environment-variables.md could not be created.");

            let mut f = BufWriter::new(output_file);

            if let Ok(lines) = read_lines(file) {
                write!(f, "{}\n{}\n", ENV_FILE_HEADERS, ENV_FILE_HEADER_SEPARATOR)
                    .expect("File could not be written.");

                let mut output = Output::new();

                for line in lines.map_while(Result::ok) {
                    if line.is_empty() || line.trim() == "#" {
                        continue;
                    }

                    let line_data = Line::new(line);

                    println!("{:?}", line_data);
                    let content = line_data.extract_content();

                    match line_data.kind {
                        LineKind::EnvVariable => {
                            // Splits env variable like NODE_ENV=development into two parts
                            if let Some((key, value)) = content.split_once('=') {
                                output.add_at(FieldKind::EnvVariable.into(), key.to_string());

                                output.add_at(FieldKind::DefaultValue.into(), value.to_string());
                            }

                            write!(f, "{}", output.as_string())
                                .expect("File could not be written.");

                            output.clear();
                        }
                        LineKind::Responsible => {
                            output.add_at(FieldKind::Responsible.into(), content)
                        }
                        LineKind::Type => output.add_at(FieldKind::Type.into(), content),
                        LineKind::Policy => output.add_at(FieldKind::Policy.into(), content),
                        LineKind::Secret => output.add_at(FieldKind::Secret.into(), content),
                        LineKind::Docs => output.add_at(FieldKind::Docs.into(), content),
                        LineKind::Description => {
                            output.add_at(FieldKind::Description.into(), content)
                        }
                    };
                }
            }
        }
    }
}
