//! Cryptography library

#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod hash;
