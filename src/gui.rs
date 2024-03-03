use crate::tracker::Tracker;

pub mod terminalgui;

pub type GuiResult<E> = Result<(), E>;

pub trait Gui {
	type Err;
	fn run(t: Tracker) -> GuiResult<Self::Err>;
}
