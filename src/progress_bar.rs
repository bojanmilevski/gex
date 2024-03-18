use indicatif::ProgressBar;
use indicatif::ProgressState;
use indicatif::ProgressStyle;
use std::fmt::Write;

pub struct Bar {
	bar: ProgressBar,
	progress: u64,
	total_size: u64,
}

impl From<u64> for Bar {
	fn from(total_size: u64) -> Self {
		let progress = 0;
		let bar = ProgressBar::new(total_size);
		let style =
			ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar:.green/black}] {bytes}/{total_bytes} ({eta})")
				.unwrap()
				.with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
					write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
				})
				.progress_chars("██ ");

		bar.set_style(style);

		Self { bar, progress, total_size }
	}
}

impl Bar {
	pub fn update(&mut self, chunk: usize) {
		let new_val = std::cmp::min(self.progress + (chunk as u64), self.total_size);
		self.progress = new_val;
		self.bar.set_position(new_val);
	}
}
