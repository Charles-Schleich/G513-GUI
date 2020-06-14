use orbtk::prelude::*;

pub use self::main_state::*;
pub use self::main_view::*;

mod main_state;
mod main_view;

const WIDTH: f64 = 1000.0; 
const HEIGHT: f64 = 700.0; 

fn main() {
    Application::from_name("G513 Keyboard")
        .window(move |ctx| {
            Window::create()
                .title("G513 Keyboard")
                .position((100.0, 100.0))
                .size(WIDTH, HEIGHT)
                .resizeable(true)
                .child(MainView::create().title("Hello OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}
