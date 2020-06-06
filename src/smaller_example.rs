use orbtk::prelude::*;

pub use self::main_state::*;
pub use self::main_view::*;
use orbtk::prelude::*;

mod main_state;
mod main_view;

fn main() {
    Application::from_name("orbtest")
        .window(move |ctx| {
            Window::create()
                .title("orbtest")
                .position((100.0, 100.0))
                .size(1000.0, 700.0)
                .resizeable(true)
                .child(MainView::create().title("Hello OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}

widget!(
    MainView<MainState> {
        title: String16
    }
);


fn generate_character_button(
    ctx: &mut BuildContext,
    id: Entity,
    sight: char,
    primary: bool,
    column: usize,
    row: usize,
) -> Entity {
    let mut button = Button::create()
        .class("single_content")
        .min_size(48.0, 48.0)
        .text(sight.to_string())
        .attach(Grid::column(column))
        .attach(Grid::row(row));

    if primary {
        button = button.class("primary");
    }

    button.build(ctx)
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {

       let mainview = self.name("MainView");
       let topgrid =  Grid::create()
                                   .columns(Columns::create()
                                                .column(500.0)
                                                .column(500.0)
                                                .build())
                                   .child(TextBlock::create()
                                                    .text("Top Left")
                                                    .element("text-block")
                                                    .class("h3")
                                                    .attach(Grid::column(0))
                                                    .build(ctx))
                                    .child(TextBox::create()
                                                    .text("Top Right")
                                                    .element("text-block")
                                                    .class("h3")
                                                    .attach(Grid::column(1))
                                                    .build(ctx));

       let botgrid = make_keyboard_layout(ctx);

       let maingrid = Grid::create()
                                    .rows(Rows::create()
                                                .row(200.0)
                                                .row(500.0)
                                                .build())
                                    .child(topgrid.attach(Grid::row(0))
                                                  .build(ctx))
                                    .child(botgrid.attach(Grid::row(1))
                                                  .build(ctx))
                                    .build(ctx);

       mainview.child(maingrid)

    }
}

fn make_keyboard_layout(ctx: &mut BuildContext) -> Grid{
    let height = 500.0;
    let width = 1000.0;
    let side_boarders = 0.0157 * (width as f64);
    let block_spacing = 0.018 * (width as f64);
    
    let mini_grid = Grid::create()
                         .columns(Columns::create()
                                        .column(width * 0.25)
                                        .column(width * 0.25)
                                        .build())
                        .child(TextBlock::create()
                                            .text("Left Bot")
                                            .element("text-block")
                                            .class("h3")
                                            .attach(Grid::column(1))
                                            .build(ctx))
                        .child(TextBlock::create()
                                            .text("Right Bot")
                                            .element("text-block")
                                            .class("h3")
                                            .attach(Grid::column(0))
                                            .build(ctx))
                        .build(ctx);

    let bottomgrid =  Grid::create()
                                 .columns(Columns::create()
                                                  .column(width * 0.5)
                                                  .column(width * 0.5)
                                                  .build())
                                 .child(TextBlock::create()
                                                        .text("Bottom Grid Here")
                                                        .element("text-block")
                                                        .class("h3")
                                                        .attach(Grid::column(1))
                                                        .build(ctx))
                                 .child(Container::create()
                                                  .element("container")
                                                  .class("content")
                                                  .padding(8.0)
                                                  .attach(Grid::row(1))
                                                  .child(mini_grid)
                                                  .build(ctx)) ;
    return bottomgrid;
}
