use std::env;

use semver::{Comparator, Op, Version};

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-env-changed=PIPEWIRE_VERSION");

	// pipewire-sys uses bindgen at compile time, so we can't rely on its compatibility
	// without explicitly knowing the installed system library version it binds to
	let explicit_pw_version: Option<String> = env::var("PIPEWIRE_VERSION").ok();
	let pw_version = match explicit_pw_version {
		Some(ver) => Some(ver),
		None => {
			let pw = pkg_config::Config::new()
				.cargo_metadata(false)
				.env_metadata(true)
				.print_system_libs(false)
				.print_system_cflags(false)
				.probe("libpipewire-0.3");
			match pw {
				Ok(pw) => Some(pw.version),
				Err(e) => {
					println!(
						"cargo:warning=Failed to detect pipewire native library version: {:?}",
						e
					);
					None
				},
			}
		},
	};
	if let Some(pw_version) = pw_version {
		let pw_version = Version::parse(&pw_version).unwrap();
		let mut req_version = Version {
			major: 0,
			minor: 3,
			patch: 0, // wp 0.4 requires pw 0.3.26
			pre: Default::default(),
			build: Default::default(),
		};
		let req = Comparator {
			op: Op::Tilde,
			major: req_version.major,
			minor: Some(req_version.minor),
			patch: Some(req_version.patch),
			pre: Default::default(),
		};
		let max_patch = if req.matches(&pw_version) { pw_version.patch } else { 43 };
		println!("cargo:rustc-cfg=pw_version=\"{}\"", pw_version);
		for patch in 0..=max_patch {
			req_version.patch = patch;
			println!("cargo:rustc-cfg=pw=\"{}\"", req_version);
		}
	}
}
