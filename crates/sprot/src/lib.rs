#![allow(rustdoc::private_intra_doc_links)]
#![deny(
    // Documentation
	// TODO: rustdoc::broken_intra_doc_links,
	// TODO: rustdoc::missing_crate_level_docs,
	// TODO: missing_docs,
	// TODO: clippy::missing_docs_in_private_items,

    // Other
	deprecated_in_future,
	exported_private_dependencies,
	future_incompatible,
	missing_copy_implementations,
	missing_debug_implementations,
	private_in_public,
	rust_2018_compatibility,
	rust_2018_idioms,
	trivial_casts,
	trivial_numeric_casts,
	//unsafe_code,
	unused_import_braces,
	unused_qualifications,
    unused_must_use,

	// clippy attributes
	clippy::missing_const_for_fn,
	clippy::redundant_pub_crate,
	clippy::use_self
)]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_alias))]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]

// TODO:
// - Rename teams to Team_one instead of colors
// - Create module/struct for strings
// - Check boxing of messages in Msg enum

pub mod error;
pub mod msg;
