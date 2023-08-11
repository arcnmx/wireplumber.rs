#![doc(html_root_url = "https://arcnmx.github.io/wireplumber.rs/v0.1.0/")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[path = "../generate/src/lib.rs"]
mod sys;

pub use sys::*;

pub type WpSpaType = libc::c_uint;
