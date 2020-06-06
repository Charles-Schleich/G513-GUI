use orbtk::prelude::*;

use crate::MainState;

widget!(
    MainView<MainState> {
        title: String16
    }
);

const _HEIGHT:f64 = 500.0;
const _WIDTH:f64 = 1000.0;


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

    let side_boarders = 0.0157 * (_WIDTH as f64);
    let horiz_boarders = 0.0535 * (_HEIGHT as f64);
    let block_spacing = 0.018 * (_WIDTH as f64);

    let f_keys = make_f_keys(ctx);
    let main_keys = make_main_keys(ctx);
    let block1 = Grid::create()
                            .rows(Rows::create()
                                        .row(horiz_boarders)
                                        .row(_HEIGHT * 0.1221 )
                                        .row(horiz_boarders)
                                        .row(_HEIGHT * 0.7175 )
                                        .row(horiz_boarders)
                                        .build())
                            .child(Container::create()
                                              .attach(Grid::row(1)) 
                                              .child(f_keys)
                                              .build(ctx))
                            .child(Container::create()
                                              .attach(Grid::row(3))
                                              .child(main_keys)
                                              .build(ctx))


                            .build(ctx);

    let bottomgrid =  Grid::create()
                                 .columns(Columns::create()
                                                  .column(side_boarders)
                                                  .column(_WIDTH * 0.6274 )
                                                  .column(block_spacing)
                                                  .column(_WIDTH * 0.1258 )
                                                  .column(block_spacing)
                                                  .column(_WIDTH * 0.168 )
                                                  .column(side_boarders)
                                                  .build())
                                 .child(Container::create()
                                              .element("container")
                                              .class("header")
                                              .attach(Grid::row(1))
                                              .child(block1)
                                              .build(ctx));
    return bottomgrid;
}

fn make_f_keys(ctx: &mut BuildContext) -> Entity{
    
    let mut fkeys =  Grid::create()
                            .columns(Columns::create()
                                             .column(_WIDTH * 0.0405 )// ESC
                                             .column(_WIDTH * 0.0315 )// ___
                                             .column(_WIDTH * 0.0405 )// F1
                                             .column(_WIDTH * 0.0405 )
                                             .column(_WIDTH * 0.0405 )
                                             .column(_WIDTH * 0.0405 )
                                             .column(_WIDTH * 0.035 ) // __
                                             .column(_WIDTH * 0.0405 )//F5
                                             .column(_WIDTH * 0.0405 )
                                             .column(_WIDTH * 0.0405 )
                                             .column(_WIDTH * 0.0405 )
                                             .column(_WIDTH * 0.035 )
                                             .column(_WIDTH * 0.0405 ) //F9
                                             .column(_WIDTH * 0.0405 )
                                             .column(_WIDTH * 0.0405 )
                                             .column(_WIDTH * 0.0405 ) //F12
                                             .build());

    let esc = generate_button(ctx,"ESC".to_string(),0);
    fkeys = fkeys.child(esc);
    let fkey_letters  = [("F1",2),("F2",3),("F3",4),("F4",5),("F5",7),("F6",8),("F7",9),("F8",10),("F9",12),("F10",13),("F11",14),("F12",15)];
    for i in fkey_letters.iter(){
        let (key,pos) = i;
        let fkey = generate_button(ctx,key.to_string(),*pos as usize);
        fkeys = fkeys.child(fkey);
    }
    return fkeys.build(ctx);
}

fn make_main_keys(ctx: &mut BuildContext) -> Entity{
    let key_w = _WIDTH * 0.0405;
    let key_h = _HEIGHT * 0.1221;
    let mainboard_HEIGHT = _HEIGHT*0.7175;

    let mut mainboard = Grid::create()
                            .rows(Rows::create()
                                        .row(mainboard_HEIGHT * 0.2)
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .build());

    // First Row 
    let first_row_keys: Vec<(&str,i32)>  = vec![("~",0) ,("1",1) ,("2",2) ,("3",3) ,("4",4) ,("5",5) ,("6",6) ,("7",7) ,("8",8) ,("9",9) ,("0",10) ,("-",11) ,("=",12) ,("BKSPC",13) ];
    let first_row= make_row(ctx, first_row_keys);

    // Second Row
    let second_row_keys: Vec<(&str,i32)> = vec![("TAB",0) ,("Q",1) ,("W",2) ,("E",3) ,("R",4) ,("T",5) ,("Y",6) ,("U",7) ,("I",8) ,("O",9) ,("P",10) ,("[",11) ,("]",12) ,("ENTER",13) ];
    let second_row= make_row(ctx, second_row_keys);

    // third Row
    let third_row_keys: Vec<(&str,i32)> = vec![("CAPS",0) ,("A",1) ,("S",2) ,("D",3) ,("F",4) ,("G",5) ,("H",6) ,("J",7) ,("K",8) ,("L",9) ,(":",10) ,("\"",11) ,("\\",12) ];
    let third_row= make_row(ctx, third_row_keys);

    // third Row
    let fourth_row_keys: Vec<(&str,i32)> = vec![("SHIFT",0) ,("Z",1) ,("X",2) ,("C",3) ,("V",4) ,("B",5) ,("N",6 ) ,("M",7 ) ,("<",8 ) ,(">",9 ) ,("?",10 ) ,("SHIFT",11 ) ];
    let fourth_row= make_row(ctx, fourth_row_keys);

    // third Row
    let fifth_row_keys: Vec<(&str,i32)> = vec![("CTRL",0) ,("WIN",1) ,("ALT",2) ,("SPACE",3) ,("ALTGR",4) ,("FN",5) ,("OPT",6 ) ,("CTRL",7 ) ,];
    let fifth_row= make_row(ctx, fifth_row_keys);

    mainboard = mainboard.child(Container::create()
                                         .attach(Grid::row(0))
                                         .child(first_row.build(ctx))
                                         .build(ctx))
                         .child(Container::create()
                                         .attach(Grid::row(1))
                                         .child(second_row.build(ctx))
                                         .build(ctx))
                         .child(Container::create()
                                         .attach(Grid::row(2))
                                         .child(third_row.build(ctx))
                                         .build(ctx))
                        .child(Container::create()
                                         .attach(Grid::row(3))
                                         .child(fourth_row.build(ctx))
                                         .build(ctx))
                        .child(Container::create()
                                         .attach(Grid::row(4))
                                         .child(fifth_row.build(ctx))
                                         .build(ctx));

    return mainboard.build(ctx);
}



fn make_row(ctx: &mut BuildContext, row_keys: Vec<(&str,i32)> ) -> Grid {
    let key_w = _WIDTH * 0.0405;
    let mut row_columns= Columns::create();
    for _ in 0..row_keys.len(){ row_columns=row_columns.column(key_w); }

    let mut row =  Grid::create().columns(row_columns.build());
    for i in row_keys.iter(){
        let (key,pos) = i;
        let fkey = generate_button(ctx,key.to_string(),*pos as usize);
        row = row.child(fkey);
    }
    row
}



fn generate_button(ctx: &mut BuildContext, name: String, pos: usize) -> Entity{
    TextBlock::create().text(name)
                       .element("text-block")
                       .class("h4")
                       .attach(Grid::column(pos))
                       .build(ctx)
}




// fn generate_operation_button(
//     ctx: &mut BuildContext,
//     id: Entity,
//     sight: char,
//     primary: bool,
//     column: usize,
//     column_span: usize,
//     row: usize,
// ) -> Entity {
//     let mut button = Button::create()
//         .class("single_content")
//         .min_size(48.0, 48.0)
//         .text(sight.to_string())
//         .class("square")
//         .on_click(move |states, _| -> bool {
//             state(id, states).action(Action::Operator(sight));
//             true
//         })
//         .attach(Grid::column(column))
//         .attach(Grid::column_span(column_span))
//         .attach(Grid::row(row));

//     if primary {
//         button = button.class("primary");
//     }
//     button.build(ctx)
// }
