#![feature(ptr_as_ref_unchecked)]
#![feature(let_chains)]
#![feature(box_into_inner)]
#![feature(string_remove_matches)]
#![feature(box_patterns)]
#![feature(pattern)]
use std::{collections::HashMap, fs};
use anyhow::{Context, Result};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "wasm", debug_assertions))]
extern crate console_error_panic_hook;

// mod frontmatter;
// mod structured_base_parser;
// mod references;
mod explain;
mod parser;
// mod toplevel;
// #[macro_use]
// mod paws_markdown;
// mod pmd_serializer;
// mod config;
mod pdf;
// mod ordered_map;
// #[cfg(feature = "text")]
// mod pmd_pure_text;
// #[cfg(feature = "html")]
// mod pmd_html;
// #[cfg(feature = "rss")]
// mod pmd_rss;
// #[cfg(feature = "pdf")]
// mod pmd_pdf;
// #[cfg(feature = "wasm")]
// mod pmd_wasm;
// #[cfg(any(feature = "wasm", feature = "html", feature = "rss", feature = "pdf"))]
// mod pmd_html_shared;

// use frontmatter::*;
// use references::*;
// use parser::*;
// use paws_markdown::*;
// use pmd_serializer::*;
// #[cfg(feature = "text")]
// use pmd_pure_text::*;
// #[cfg(feature = "html")]
// use pmd_html::*;
// #[cfg(feature = "rss")]
// use pmd_rss::*;
// #[cfg(feature = "pdf")]
// use pmd_pdf::*;
