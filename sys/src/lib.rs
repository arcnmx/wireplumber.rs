#![doc(html_root_url = "https://arcnmx.github.io/wireplumber.rs/")]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[path = "../../generate/src/lib.rs"]
mod sys;

pub use sys::*;

pub type WpSpaType = libc::c_uint;
