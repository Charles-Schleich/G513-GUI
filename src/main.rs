use orbtk::prelude::*;

pub use self::main_state::*;
pub use self::main_view::*;

mod main_state;
mod main_view;

fn main() {
    Application::from_name("G513 Keyboard")
        .window(move |ctx| {
            Window::create()
                .title("G513 Keyboard")
                .position((100.0, 100.0))
                .size(1000.0, 700.0)
                .resizeable(true)
                .child(MainView::create().title("Hello OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}
