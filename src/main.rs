use orbtk::prelude::*;

pub use self::main_state::*;
pub use self::main_view::*;

mod main_state;
mod main_view;

const WIDTH: f64 = 1300.0; 
const HEIGHT: f64 = 700.0; 

fn main() {
    Application::from_name("G513 Keyboard")
        .window(move |ctx| {
            Window::create()
                .title("G513 Keyboard")
                .position((100.0, 100.0))
                .size(WIDTH, HEIGHT)
                .resizeable(false)
                .child(MainView::create().title("G513 Keyboard").build(ctx))
                .build(ctx)
        })
        .run();
}
