mod dto;
mod storage;

pub use dto::{AppSession, DrawPrimitiveDto, EditorSession, PaintToolDto};
pub use storage::{load_session, save_session};
