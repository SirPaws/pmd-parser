#![feature(ptr_as_ref_unchecked)]
#![feature(let_chains)]
#![feature(box_into_inner)]
#![feature(string_remove_matches)]
#![feature(box_patterns)]
#![feature(pattern)]
#[allow(dead_code)]
#[allow(unused)]
use std::{collections::HashMap, fs};
use anyhow::{Context, Result};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "wasm", debug_assertions))]
extern crate console_error_panic_hook;

// mod frontmatter;
mod parser;
// mod toplevel;
mod pdf;

// use frontmatter::*;
use parser::*;
// use paws_markdown::*;
// use pmd_serializer::*;

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)] out_dir: Option<String>, 

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "html")]
    Html {files: Vec<PathBuf> },

    #[cfg(feature = "rss")]
    Rss  {files: Vec<PathBuf> },
    
    #[cfg(feature = "text")]
    Text  {files: Vec<PathBuf> },

    #[cfg(feature = "pdf")]
    Pdf   {files: Vec<PathBuf> },
    
    #[cfg(feature = "text")]
    Paragraph {file: PathBuf },
    #[cfg(feature = "text")]
    Subtitle  {file: PathBuf },

    Explain {
        feature: Option<String>,
        extra: Option<String>,
    },
    List {
        option: Option<String>
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {
    let cli = Cli::parse();
    let dir = cli.out_dir.unwrap_or("./out".into());
    if !Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.clone())?; 
    }

    match &cli.command {
        #[cfg(feature = "text")]
        Commands::Paragraph { file } => {
            let result = parse_file(&file.to_str().context("expected a file")?.to_string())?;
            let paragraph = result.body.iter().find(|(x, _)| match x { Element::Paragraph(_) => true, _ => false});
            if let Some((Element::Paragraph(content), _)) = paragraph {
                // let mut serialiser = PMDPureTextSerializer::new();
                // serialiser.references = result.references.clone();
                // let text = serialiser.convert_paragraph(&content, &String::new())?;
                // println!("{text}");
            } else {
            }
        },
        #[cfg(feature = "text")]
        Commands::Subtitle { file } => {
            let result = parse_file(&file.to_str().context("expected a file")?.to_string())?;
            let text = result.meta.subtitle;
            println!("{text}");
        },
        #[cfg(feature = "html")]
        Commands::Html{files} => {
            let out_dir = Path::new(dir.as_str());
            for file in files {
                let stem = file.as_path().file_stem().context("expected file name")?;
                let mut out_file = out_dir.join(stem);
                out_file.set_extension("html");
                println!("outputting to file {}", out_file.to_str().expect("whatever"));

                let result = parse_file(&file.to_str().context("expected a file")?.to_string())?;
                println!("{:?}", result);
                // let html = to_string_from_boxed(&result, PMDHTMLSerializer::new(stem.to_str().context("converting OsStr to str")?))?;
                // if out_file.exists() {
                //     fs::remove_file(&out_file)?;
                // }
                // fs::write(out_file, html)?;
            }
        }, 
        #[cfg(feature = "pdf")]
        Commands::Pdf{files} => {
            let out_dir = Path::new(dir.as_str());
            for file in files {
                let stem = file.as_path().file_stem().context("expected file name")?;
                let stem = format!("pdf-{}", stem.to_str().expect("could not convert filename to str"));
                let mut out_file = out_dir.join(stem.clone());
                out_file.set_extension("pdf");
                println!("outputting to file {}", out_file.to_str().expect("whatever"));

                let result = parse_file(&file.to_str().context("expected a file")?.to_string())?;
                println!("{:?}", result);
                // let html   = to_string_from_boxed(&result, PMDPDFSerializer::new(stem.as_str()))?;
                // if out_file.exists() {
                //     fs::remove_file(&out_file)?;
                // }
                // fs::write(out_file, html)?;
            }
        }, 
        #[cfg(feature = "rss")]
        Commands::Rss{files}  => {
            let out_dir = Path::new(dir.as_str());
            for file in files {
                let stem = file.as_path().file_stem().context("expected file name")?;
                let mut out_file = out_dir.join(stem);
                out_file.set_extension("rss");

                let result = parse_file(&file.to_str().context("expected a file")?.to_string())?;
                println!("{:?}", result);
                // let html   = to_string(&result, PMDRSSSerializer::new(stem.to_str().context("converting OsStr to str")?))?;
                // if out_file.exists() {
                //     fs::remove_file(&out_file)?;
                // }
                // fs::write(out_file, html)?;
            }
        }, 
        #[cfg(feature = "text")]
        Commands::Text{files} => {
            for file in files {
                let file_path_string = file.as_path().to_str().expect("expected a valid path");
                // println!("// {} //////////////////////////////////////////////////////////////////////////", file_path_string);

                let result = parse_file(&file.to_str().context("expected a file")?.to_string())?;
                println!("{:?}", result);
                // let text   = to_string(&result, PMDPureTextSerializer::new())?;
                // 
                // println!("{text}");
                // 
                // println!("////////////////////////////////////////////////////////////////////////////////");
            }
        }
        Commands::Explain{feature, extra}  => {
            // explain(feature, extra);
        },
        Commands::List {option} => {
            let print_all = ||{
                println!("Arguments:");
                println!("    [Feature]");
                println!();
                println!("Features:");      
                println!("    frontmatter           ---");
                println!("    citation              £some-citation");
            };

            if let Some(option) = option {
                match option.to_lowercase().as_str() {
                    "frontmatter" => {
                        println!("Frontmatter Keys:");
                        println!("    title");
                        println!("    subtitle");
                        println!("    banner");
                        println!("    ");
                        println!("    url");
                        println!("    base-url");
                        println!("    base_url");
                        println!("    ");
                        println!("    data");
                        println!("    data-dir");
                        println!("    data_dir");
                        println!("    data dir");
                        println!("    ");
                        println!("    blog");
                        println!("    blog-dir");
                        println!("    blog_dir");
                        println!("    blog dir");
                        println!("    ");
                        println!("    notes-title");
                        println!("    ");
                        println!("    bibliography-title");
                        println!("    references-title");
                        println!("    sources-title");
                        println!("    bibliography title");
                        println!("    references title");
                        println!("    sources title");
                        println!("    bibliography_title");
                        println!("    references_title");
                        println!("    sources_title");
                        println!("    ");
                        println!("    date");
                        println!("    date written");
                        println!("    date-written");
                        println!("    date_written");
                        println!("    ");
                        println!("    last-update");
                        println!("    last_update");
                        println!("    last update");
                        println!("    last-updated");
                        println!("    last_updated");
                        println!("    last updated");
                        println!("    ");
                        println!("    cite-contacts");
                        println!("    ");
                        println!("    pdf-no-first-page removes the first page and adds title/subtitle to the document");
                        println!("    pdf-text-size     sets the font size for paragraphs");
                        println!("    pdf-line-height   sets the line height");
                        println!("    pdf-font          changes the font");
                        println!("    ");
                        println!("    pdf-header        inserts text into the header, centered");
                        println!("    pdf-header-left   inserts text into the header, left aligned");
                        println!("    pdf-header-center inserts text into the header, centered");
                        println!("    pdf-header-right  inserts text into the header, right aligned");
                        println!("    ");
                        println!("    pdf-footer        inserts text into the footer, centered");
                        println!("    pdf-footer-left   inserts text into the footer, left aligned");
                        println!("    pdf-footer-center inserts text into the footer, centered");
                        println!("    pdf-footer-right  inserts text into the footer, right aligned");
                    },
                    "citation"    => {
                        println!("Citation Keys:");
                        println!("    title");
                        println!("    description");
                        println!("    ");
                        println!("    container-title");
                        println!("    publisher");
                        println!("    edition");
                        println!("    version");
                        println!("    issue");
                        println!("    volume");
                        println!("    pages");
                        println!("    link");
                        println!("    doi");
                        println!("    esbn");
                        println!("    ");
                        println!("    date");
                        println!("    day");
                        println!("    month");
                        println!("    year");
                        println!("    ");
                        println!("    date-retrieved");
                        println!("    day-retrieved");
                        println!("    month-retrieved");
                        println!("    year-retrieved");
                        println!("    ");
                        println!("    author");
                        println!("    authors");
                        println!("    ");
                        println!("    editor");
                        println!("    editors");
                        println!("    ");
                        println!("    translator");
                        println!("    translators");
                    },
                    _ => print_all()
                }
            } else {
                print_all()
            }
        }
    }
    Ok(())
}

/*
#[cfg(not(feature = "wasm"))]
fn main() -> Result<()> {
    execute()
}

#[cfg(feature = "wasm")]
use pmd_wasm::PMDWASMSerializer;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn generate_output(source: &str, pdf: bool) -> std::result::Result<JsValue, wasm_bindgen::JsError> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let result = parse(&source.to_string(), None).map_err(|e| JsError::from(&*e))?;
    let html = to_string(&result, PMDWASMSerializer::new(pdf)).map_err(|e| JsError::from(&*e))?;
    Ok(JsValue::from_str(&html))
}

#[cfg(not(feature = "wasm"))]
macro_rules! build_executable {
    ($($body: tt)*) => { $($body)* }
}

#[cfg(feature = "wasm")]
macro_rules! build_executable {
    ($($body: tt)*) => { }
}

build_executable!{
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

use crate::{explain::explain, file_parse, to_string, BlogBody};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)] out_dir: Option<String>, 

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "html")]
    Html {files: Vec<PathBuf> },

    #[cfg(feature = "rss")]
    Rss  {files: Vec<PathBuf> },
    
    #[cfg(feature = "text")]
    Text  {files: Vec<PathBuf> },

    #[cfg(feature = "pdf")]
    Pdf   {files: Vec<PathBuf> },
    
    #[cfg(feature = "text")]
    Paragraph {file: PathBuf },
    #[cfg(feature = "text")]
    Subtitle  {file: PathBuf },

    Explain {
        feature: Option<String>,
        extra: Option<String>,
    },
    List {
        option: Option<String>
    }
}



pub fn execute() -> Result<()> {
    let cli = Cli::parse();
    let dir = cli.out_dir.unwrap_or("./out".into());
    if !Path::new(dir.as_str()).exists() {
        fs::create_dir_all(dir.clone())?; 
    }

    match &cli.command {
        #[cfg(feature = "text")]
        Commands::Paragraph { file } => {
            let result = file_parse(&file.to_str().context("expected a file")?.to_string())?;
            let paragraph = result.body.iter().find(|(x, _)| match x { BlogBody::Paragraph(_) => true, _ => false});
            if let Some((BlogBody::Paragraph(content), _)) = paragraph {
                let mut serialiser = PMDPureTextSerializer::new();
                serialiser.references = result.references.clone();
                let text = serialiser.convert_paragraph(&content, &String::new())?;
                println!("{text}");
            } else {
            }
        },
        #[cfg(feature = "text")]
        Commands::Subtitle { file } => {
            let result = file_parse(&file.to_str().context("expected a file")?.to_string())?;
            let text = result.header.subtitle;
            println!("{text}");
        },
        #[cfg(feature = "html")]
        Commands::Html{files} => {
            let out_dir = Path::new(dir.as_str());
            for file in files {
                let stem = file.as_path().file_stem().context("expected file name")?;
                let mut out_file = out_dir.join(stem);
                out_file.set_extension("html");
                println!("outputting to file {}", out_file.to_str().expect("whatever"));

                let result = file_parse(&file.to_str().context("expected a file")?.to_string())?;
                let html = to_string_from_boxed(&result, PMDHTMLSerializer::new(stem.to_str().context("converting OsStr to str")?))?;
                if out_file.exists() {
                    fs::remove_file(&out_file)?;
                }
                fs::write(out_file, html)?;
            }
        }, 
        #[cfg(feature = "pdf")]
        Commands::Pdf{files} => {
            let out_dir = Path::new(dir.as_str());
            for file in files {
                let stem = file.as_path().file_stem().context("expected file name")?;
                let stem = format!("pdf-{}", stem.to_str().expect("could not convert filename to str"));
                let mut out_file = out_dir.join(stem.clone());
                out_file.set_extension("pdf");
                println!("outputting to file {}", out_file.to_str().expect("whatever"));

                let result = file_parse(&file.to_str().context("expected a file")?.to_string())?;
                let html   = to_string_from_boxed(&result, PMDPDFSerializer::new(stem.as_str()))?;
                if out_file.exists() {
                    fs::remove_file(&out_file)?;
                }
                fs::write(out_file, html)?;
            }
        }, 
        #[cfg(feature = "rss")]
        Commands::Rss{files}  => {
            let out_dir = Path::new(dir.as_str());
            for file in files {
                let stem = file.as_path().file_stem().context("expected file name")?;
                let mut out_file = out_dir.join(stem);
                out_file.set_extension("rss");

                let result = file_parse(&file.to_str().context("expected a file")?.to_string())?;
                let html   = to_string(&result, PMDRSSSerializer::new(stem.to_str().context("converting OsStr to str")?))?;
                if out_file.exists() {
                    fs::remove_file(&out_file)?;
                }
                fs::write(out_file, html)?;
            }
        }, 
        #[cfg(feature = "text")]
        Commands::Text{files} => {
            for file in files {
                let file_path_string = file.as_path().to_str().expect("expected a valid path");
                println!("// {} //////////////////////////////////////////////////////////////////////////", file_path_string);

                let result = file_parse(&file.to_str().context("expected a file")?.to_string())?;
                let text   = to_string(&result, PMDPureTextSerializer::new())?;

                println!("{text}");

                println!("////////////////////////////////////////////////////////////////////////////////");
            }
        }
        Commands::Explain{feature, extra}  => {
            explain(feature, extra);
        },
        Commands::List {option} => {
            let print_all = ||{
                println!("Arguments:");
                println!("    [Feature]");
                println!();
                println!("Features:");      
                println!("    frontmatter           ---");
                println!("    citation              £some-citation");
            };

            if let Some(option) = option {
                match option.to_lowercase().as_str() {
                    "frontmatter" => {
                        println!("Frontmatter Keys:");
                        println!("    title");
                        println!("    subtitle");
                        println!("    banner");
                        println!("    ");
                        println!("    url");
                        println!("    base-url");
                        println!("    base_url");
                        println!("    ");
                        println!("    data");
                        println!("    data-dir");
                        println!("    data_dir");
                        println!("    data dir");
                        println!("    ");
                        println!("    blog");
                        println!("    blog-dir");
                        println!("    blog_dir");
                        println!("    blog dir");
                        println!("    ");
                        println!("    notes-title");
                        println!("    ");
                        println!("    bibliography-title");
                        println!("    references-title");
                        println!("    sources-title");
                        println!("    bibliography title");
                        println!("    references title");
                        println!("    sources title");
                        println!("    bibliography_title");
                        println!("    references_title");
                        println!("    sources_title");
                        println!("    ");
                        println!("    date");
                        println!("    date written");
                        println!("    date-written");
                        println!("    date_written");
                        println!("    ");
                        println!("    last-update");
                        println!("    last_update");
                        println!("    last update");
                        println!("    last-updated");
                        println!("    last_updated");
                        println!("    last updated");
                        println!("    ");
                        println!("    cite-contacts");
                        println!("    ");
                        println!("    pdf-no-first-page removes the first page and adds title/subtitle to the document");
                        println!("    pdf-text-size     sets the font size for paragraphs");
                        println!("    pdf-line-height   sets the line height");
                        println!("    pdf-font          changes the font");
                        println!("    ");
                        println!("    pdf-header        inserts text into the header, centered");
                        println!("    pdf-header-left   inserts text into the header, left aligned");
                        println!("    pdf-header-center inserts text into the header, centered");
                        println!("    pdf-header-right  inserts text into the header, right aligned");
                        println!("    ");
                        println!("    pdf-footer        inserts text into the footer, centered");
                        println!("    pdf-footer-left   inserts text into the footer, left aligned");
                        println!("    pdf-footer-center inserts text into the footer, centered");
                        println!("    pdf-footer-right  inserts text into the footer, right aligned");
                    },
                    "citation"    => {
                        println!("Citation Keys:");
                        println!("    title");
                        println!("    description");
                        println!("    ");
                        println!("    container-title");
                        println!("    publisher");
                        println!("    edition");
                        println!("    version");
                        println!("    issue");
                        println!("    volume");
                        println!("    pages");
                        println!("    link");
                        println!("    doi");
                        println!("    esbn");
                        println!("    ");
                        println!("    date");
                        println!("    day");
                        println!("    month");
                        println!("    year");
                        println!("    ");
                        println!("    date-retrieved");
                        println!("    day-retrieved");
                        println!("    month-retrieved");
                        println!("    year-retrieved");
                        println!("    ");
                        println!("    author");
                        println!("    authors");
                        println!("    ");
                        println!("    editor");
                        println!("    editors");
                        println!("    ");
                        println!("    translator");
                        println!("    translators");
                    },
                    _ => print_all()
                }
            } else {
                print_all()
            }
        }
    }
    Ok(())
}

}
*/

