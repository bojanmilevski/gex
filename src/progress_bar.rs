use anyhow::Context;
use anyhow::Result;
use indicatif::ProgressBar;
use indicatif::ProgressState;
use indicatif::ProgressStyle;
use std::fmt::Write;

pub struct Bar {
	bar: ProgressBar,
	progress: u64,
	total_size: u64,
}

impl TryFrom<u64> for Bar {
	type Error = anyhow::Error;

	fn try_from(total_size: u64) -> Result<Self> {
		let progress = 0;
		let bar = ProgressBar::new(total_size);
		let style =
			ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar:.green/black}] {bytes}/{total_bytes} ({eta})")
				.context("Invalid progress bar template.")?
				.with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
					write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
				})
				.progress_chars("██ ");

		bar.set_style(style);

		let bar = Self { bar, progress, total_size };

		Ok(bar)
	}
}

impl Bar {
	pub fn update(&mut self, chunk: usize) {
		let new_val = std::cmp::min(self.progress + (chunk as u64), self.total_size);
		self.progress = new_val;
		self.bar.set_position(new_val);
	}
}
