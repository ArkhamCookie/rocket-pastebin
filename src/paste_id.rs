use std::borrow::Cow;
use std::path::{Path, PathBuf};

use rand::{thread_rng, Rng};

/// A *probaly* unique paste ID.
pub struct PasteID<'a>(Cow<'a, str>);

impl PasteID<'_> {
	/// Generate a *probably* unique ID with `size` characters.
	/// For readability, the cahracters used are from the sets [0-9], [A-Z], [a-z].
	/// The probability of collision depends on the value of `size`
	/// and the number of IDs generated thus far.
	pub fn new(size: usize) -> PasteID<'static> {
		const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

		let mut id = String::with_capacity(size);
		let mut rng = thread_rng();

		for _ in 0..size {
			id.push(BASE62[rng.gen::<usize>() % 62] as char);
		}

		PasteID(Cow::Owned(id))
	}

	/// Return the path to the paste in `uploads/` corresponding to this ID.
	pub fn file_path(&self) -> PathBuf {
		let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads");
		Path::new(root).join(self.0.as_ref())
	}
}
