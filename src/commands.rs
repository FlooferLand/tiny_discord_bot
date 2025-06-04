use crate::Context;

pub mod info;
pub mod save;
pub mod char;
pub mod config;

pub async fn pre_command<'a>(ctx: Context<'a>) {
	#[cfg(feature = "profiling")] puffin::GlobalProfiler::lock().new_frame();
}

pub async fn post_command<'a>(ctx: Context<'a>) {
	
}
