//! The module for generating Solidity documentation.
//!
//! See [`DocBuilder`].

#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
extern crate foundry_common;

#[macro_use]
extern crate tracing;

mod builder;
pub use builder::DocBuilder;

mod document;
pub use document::Document;

mod helpers;

mod parser;
pub use parser::{
    Comment, CommentTag, Comments, CommentsRef, ParseItem, ParseSource, Parser, error,
};

mod preprocessor;
pub use preprocessor::*;

mod writer;
pub use writer::{AsDoc, AsDocResult, BufWriter, Markdown};

pub use mdbook;
