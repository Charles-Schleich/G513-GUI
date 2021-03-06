use orbtk::prelude::*;

use std::process::Command;
use crate::MainState;

widget!(
    MainView<MainState> {
        title: String16
    }
);

const _HEIGHT:f64 = 500.0;
const _WIDTH:f64 = 1300.0;

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {

        let mainview = self.name("MainView");

        let colSel = make_colour_select(ctx);

        let topgrid =  Grid::create()
                                    .columns(Columns::create()
                                                    .column(500.0)
                                                    .column(500.0)
                                                    .build())
                                    .child(colSel.attach(Grid::column(1))
                                                 .build(ctx))
                                    .child(TextBox::create()
                                                        .text("Top Right")
                                                        .element("text-block")
                                                        .class("h3")
                                                        .attach(Grid::column(0))
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



//    _____      _                     _____      _           _   
//   / ____|    | |                   / ____|    | |         | |  
//  | |     ___ | | ___  _   _ _ __  | (___   ___| | ___  ___| |_ 
//  | |    / _ \| |/ _ \| | | | '__|  \___ \ / _ \ |/ _ \/ __| __|
//  | |___| (_) | | (_) | |_| | |     ____) |  __/ |  __/ (__| |_ 
//   \_____\___/|_|\___/ \__,_|_|    |_____/ \___|_|\___|\___|\__|


fn make_colour_select(ctx: &mut BuildContext) -> Grid {
    let local_height = 200.0;
    
    let red =  make_colour_slider(ctx, String::from("Red"));
    let green =  make_colour_slider(ctx, String::from("Green"));
    let blue =  make_colour_slider(ctx, String::from("Blue"));

    let col_sel =  Grid::create()
                        .rows(Rows::create()
                                        .row(local_height/3.0)
                                        .row(local_height/3.0)
                                        .row(local_height/3.0)
                                        .build())
                        .child(red.attach(Grid::row(0))
                                  .build(ctx))
                        .child(green.attach(Grid::row(1))
                                    .build(ctx))
                        .child(blue.attach(Grid::row(2))
                                   .build(ctx));
    

    col_sel
}


fn make_colour_slider(ctx: &mut BuildContext, colour:String) -> Grid {
    let local_width = _WIDTH/2.0;
    let mut col_val_id = colour.clone();
    col_val_id.push_str("_val");
    let colour_slider =  Grid::create()
                        .columns(Columns::create()
                                        .column(local_width/3.0)
                                        .column(local_width/3.0)
                                        .column(local_width/3.0)
                                        .build())
                        .child(TextBlock::create()
                                         .text(colour)
                                         .class("h3")
                                         .attach(Grid::column(0))
                                         .attach(Grid::row(0))
                                         .build(ctx))
                        .child(Slider::create()
                                        .attach(Grid::column(1))
                                        .attach(Grid::row(0))
                                        .minimum(0)
                                        .minimum(255)
                                        .on_changed(move |states, entity| {
                                            // let val =((*ctx.get_widget(entity).get::<f64>("val")).floor() as i32).to_string();
                                        })
                                        .build(ctx))
                        .child(TextBlock::create()
                                         .text("0")
                                         .id(col_val_id)
                                         .class("h3")
                                         .attach(Grid::column(2))
                                         .build(ctx));
    colour_slider
}




//   __  __       _         _  __          _                         _ 
//  |  \/  |     (_)       | |/ /         | |                       | |
//  | \  / | __ _ _ _ __   | ' / ___ _   _| |__   ___   __ _ _ __ __| |
//  | |\/| |/ _` | | '_ \  |  < / _ \ | | | '_ \ / _ \ / _` | '__/ _` |
//  | |  | | (_| | | | | | | . \  __/ |_| | |_) | (_) | (_| | | | (_| |
//  |_|  |_|\__,_|_|_| |_| |_|\_\___|\__, |_.__/ \___/ \__,_|_|  \__,_|
//                                    __/ |                            
//                                   |___/                

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


    let prtlckpause= make_prt_lock_pause(ctx);
    let home_arrows = make_home_arrows(ctx);
    let block2 = Grid::create()
                            .rows(Rows::create()
                                        .row(horiz_boarders)
                                        .row(_HEIGHT * 0.1221 )
                                        .row(horiz_boarders)
                                        .row(_HEIGHT * 0.7175 )
                                        .row(horiz_boarders)
                                        .build())
                            .child(Container::create()
                                              .attach(Grid::row(1)) 
                                              .child(prtlckpause)
                                              .build(ctx))
                            .child(Container::create()
                                              .attach(Grid::row(3))
                                              .child(home_arrows)
                                              .build(ctx))
                            .build(ctx);

    let num_pad = make_num_pad(ctx);

    let block3 = Grid::create()
                            .rows(Rows::create()
                                        .row(horiz_boarders)
                                        .row(_HEIGHT * 0.1221 )
                                        .row(horiz_boarders)
                                        .row(_HEIGHT * 0.7175 )
                                        .row(horiz_boarders)
                                        .build())
                            .child(Container::create()
                                              .attach(Grid::row(3))
                                              .child(num_pad)
                                              .build(ctx))
                            .build(ctx);


    let bottomgrid =  Grid::create()
                                 .columns(Columns::create()
                                                  .column(side_boarders)
                                                  .column(_WIDTH * 0.6274 )
                                                  .column(0.030 * (_WIDTH as f64))
                                                  .column(_WIDTH * 0.1258 )
                                                  .column(block_spacing)
                                                  .column(_WIDTH * 0.168 )
                                                  .column(side_boarders)
                                                  .build())
                                 .child(Container::create()
                                              .element("container")
                                              .class("header")
                                              .attach(Grid::column(1))
                                              .child(block1)
                                              .build(ctx))
                                 .child(Container::create()
                                              .element("container")
                                              .class("header")
                                              .attach(Grid::column(3))
                                              .child(block2)
                                              .build(ctx))
                                 .child(Container::create()
                                              .element("container")
                                              .class("header")
                                              .attach(Grid::column(5))
                                              .child(block3)
                                              .build(ctx))
                                ;

    return bottomgrid;
}

//   ____    _                  _        __   
//  |  _ \  | |                | |      /_ |  
//  | |_) | | |   ___     ___  | | __    | |  
//  |  _ <  | |  / _ \   / __| | |/ /    | |  
//  | |_) | | | | (_) | | (__  |   <     | |  
//  |____/  |_|  \___/   \___| |_|\_\    |_|  
                                           
                                          

fn make_f_keys(ctx: &mut BuildContext) -> Entity {
    let kw = 0.0 ; // Standard Key width IGNORED IN F KEYS 
    let key_colsize = _WIDTH * 0.035 ;
    let cols = Columns::create()
                                .column(key_colsize).column(6.0)// ESC
                                .column(_WIDTH * 0.0315 ).column(6.0)// ___
                                .column(key_colsize).column(6.0)// F1
                                .column(key_colsize).column(6.0)
                                .column(key_colsize).column(6.0)
                                .column(key_colsize).column(6.0)
                                .column(key_colsize).column(6.0) // __
                                .column(key_colsize).column(6.0)//F5
                                .column(key_colsize).column(6.0)
                                .column(key_colsize).column(6.0)
                                .column(key_colsize).column(6.0)
                                .column(key_colsize).column(6.0)
                                .column(key_colsize).column(6.0) //F9
                                .column(key_colsize).column(6.0)
                                .column(key_colsize).column(6.0)
                                .column(key_colsize).column(6.0) //F12
                                .build();


    let mut fkeys =  Grid::create()
                            .columns(cols);

    let esc = generate_character_button(ctx,"ESC".to_string(),"ESC".to_string(),0,kw);
    fkeys = fkeys.child(esc);

    let fkey_letters  = [("F1",Some("F1"),2,kw),("F2",None,3,kw),("F3",None,4,kw),("F4",None,5,kw),("F5",None,7,kw),("F6",None,8,kw),("F7",None,9,kw),("F8",None,10,kw),("F9",None,12,kw),("F10",None,13,kw),("F11",None,14,kw),("F12",None,15,kw)];

    for i in fkey_letters.iter(){
        let (key,key_value,pos,width ) = i;
        let key_value:String = match key_value{
            None => {key.clone().to_string()}
            Some(x) => {x.to_string().clone()}
        };
        let fkey = generate_character_button(ctx, key.to_string(),key_value,*pos as usize * 2 , *width);
        fkeys = fkeys.child(fkey);
    }
    return fkeys.build(ctx);
}


fn make_main_keys(ctx: &mut BuildContext) -> Entity{
    let key_w = _WIDTH * 0.0405;
    let key_h = _HEIGHT * 0.1221;
    let mainboard_HEIGHT = _HEIGHT*0.7175;
    let kw = 48.0 ; //  Standard Key width

    let mut mainboard = Grid::create()
                            .rows(Rows::create()
                                        .row(mainboard_HEIGHT * 0.2)
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .build());

    // First Row
    
    let first_row_keys:  Vec<(&str,Option<&str>,i32,f64)>  = vec![("~",None,0,kw) ,("1",None,1,kw) ,("2",None,2,kw) ,("3",None,3,kw) ,("4",None,4,kw) ,("5",None,5,kw) ,("6",None,6,kw) ,("7",None,7,kw) ,("8",None,8,kw) ,("9",None,9,kw) ,("0",None,10,kw) ,("-",None,11,kw) ,("=",None,12,kw) ,("BKSPC",None,13,115.0) ];
    let first_row= make_row(ctx, first_row_keys);

    // // Second Row
    let second_row_keys:  Vec<(&str,Option<&str>,i32,f64)>  = vec![("TAB",None,0,66.66) ,("Q",None,1,kw) ,("W",None,2,kw) ,("E",None,3,kw) ,("R",None,4,kw) ,("T",None,5,kw) ,("Y",None,6,kw) ,("U",None,7,kw) ,("I",None,8,kw) ,("O",None,9,kw) ,("P",None,10,kw) ,("[",None,11,kw) ,("]",None,12,kw) ,("ENTER",None,13,70.0) ];
    let second_row= make_row(ctx, second_row_keys);

    // // third Row
    let third_row_keys:  Vec<(&str,Option<&str>,i32,f64)>  = vec![("CAPS",None,0,110.0) ,("A",None,1,kw) ,("S",None,2,kw) ,("D",None,3,kw) ,("F",None,4,kw) ,("G",None,5,kw) ,("H",None,6,kw) ,("J",None,7,kw) ,("K",None,8,kw) ,("L",None,9,kw) ,(":",None,10,kw) ,("\"",None,11,kw) ,("\\",Some("\\\\"),12,kw)];
    let third_row= make_row(ctx, third_row_keys);

    // // Fourth Row
    let fourth_row_keys:  Vec<(&str,Option<&str>,i32,f64)>  = vec![("LSHIFT",None,0,140.0) ,("Z",None,1,kw) ,("X",None,2,kw) ,("C",None,3,kw) ,("V",None,4,kw) ,("B",None,5,kw) ,("N",None,6 ,kw) ,("M",None,7 ,kw) ,("<",None,8 ,kw) ,(">",None,9 ,kw) ,("?",None,10 ,kw) ,("RSHIFT",None,11 ,130.67) ];
    let fourth_row= make_row(ctx, fourth_row_keys);

    // // Fifth Row
    let bk = 56.0;
    let fifth_row_keys:  Vec<(&str,Option<&str>,i32,f64)>  = vec![("LCTRL",None,0,66.66) ,("WIN",None,1,bk) ,("ALT",Some("alt_left"),2,bk) ,("SPACE",None,3,270.0) ,("ALTGR",None,4,bk) ,("FN",None,5,bk) ,("MENU",Some("menu"),6 ,bk) ,("RCTRL",None,7 ,66.66)];
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

//   ____    _                  _        ___  
//  |  _ \  | |                | |      |__ \ 
//  | |_) | | |   ___     ___  | | __      ) |
//  |  _ <  | |  / _ \   / __| | |/ /     / / 
//  | |_) | | | | (_) | | (__  |   <     / /_ 
//  |____/  |_|  \___/   \___| |_|\_\   |____|
                                          

fn make_prt_lock_pause(ctx: &mut BuildContext) -> Entity {
    let kw = 48.0 ; //  Standard Key width
    let key_w = _WIDTH * 0.040;

    let cols = Columns::create()
                                .column(key_w).column(6.0) // PRTSC
                                .column(key_w).column(6.0) // SCRLCK
                                .column(key_w).column(6.0) // PAUSE
                                .build();

    let mut keys =  Grid::create()
                            .columns(cols);


    let fkey_letters  = [("PRTSC",Some("print_screen"),3,kw),("SCRLK",Some("scroll_lock"),2,kw),("PAUSE",None,1,kw)];

    for i in fkey_letters.iter(){
        let (key,key_value,pos,width ) = i;   
        let key_value:String = match key_value{
            None => {key.clone().to_string()}
            Some(x) => {x.to_string().clone()}
        };

        let fkey = generate_character_button(ctx, key.to_string(),key_value,*pos as usize * 2 , *width);
        keys = keys.child(fkey);
    }
    return keys.build(ctx);
}


fn make_home_arrows(ctx: &mut BuildContext) -> Entity{

    let mainboard_HEIGHT = _HEIGHT*0.7175;
    let kw = 40.0 ; //  Standard Key width
    let mut arrowkeys = Grid::create()
                            .rows(Rows::create()
                                        .row(mainboard_HEIGHT * 0.2)
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .build());

    let key_w = _WIDTH * 0.040;
    let cols = Columns::create()
                                .column(key_w).column(6.0) // PRTSC
                                .column(key_w).column(6.0) // SCRLCK
                                .column(key_w).column(6.0) // PAUSE
                                .build();

    
    // First Row
    let first_row_keys:  Vec<(&str,Option<&str>,i32,f64)>  = vec![("INS",None,0,10.0) ,("HOME",None,1,kw) ,("PG UP",Some("page_up"),2,kw)  ];
    let first_row = make_row_skinny(ctx, first_row_keys, cols.clone());
    
    // Second Row
    let second_row_keys: Vec<(&str,Option<&str>,i32,f64)>  = vec![("DEL",None,0,kw) ,("END",None,1,kw) ,("PG DN",Some("page_down"),2,kw) ];
    let second_row = make_row_skinny(ctx,second_row_keys,cols.clone());
    
    // third Row
    let third_row_keys:  Vec<(&str,Option<&str>,i32,f64)>  = vec![ ];
    let third_row= make_row_skinny(ctx, third_row_keys,cols.clone());

    // Fourth Row
    let fourth_row_keys: Vec<(&str,Option<&str>,i32,f64)>  = vec![("UP",Some("arrow_top"),1,kw)];
    let fourth_row = make_row_skinny(ctx,fourth_row_keys,cols.clone());
    
    // Fifth Row
    let fifth_row_keys:  Vec<(&str,Option<&str>,i32,f64)>  = vec![("LEFT",Some("arrow_left"),0,kw) ,("DOWN",Some("arrow_bottom"),1,kw) ,("RIGHT",Some("arrow_right"),2,kw) ];
    let fifth_row = make_row_skinny(ctx,fifth_row_keys,cols.clone());
    
    arrowkeys = arrowkeys.child(Container::create()
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

    return arrowkeys.build(ctx);
}


//   ____    _                  _        ____  
//  |  _ \  | |                | |      |___ \ 
//  | |_) | | |   ___     ___  | | __     __) |
//  |  _ <  | |  / _ \   / __| | |/ /    |__ < 
//  | |_) | | | | (_) | | (__  |   <     ___) |
//  |____/  |_|  \___/   \___| |_|\_\   |____/ 
                                            
                                         

fn make_num_pad(ctx: &mut BuildContext) -> Entity{
    // Numpad Num pad
    let mainboard_HEIGHT = _HEIGHT*0.7175;
    let kw = 40.0 ; //  Standard Key width
    let mut arrowkeys = Grid::create()
                            .rows(Rows::create()
                                        .row(mainboard_HEIGHT * 0.2)
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .row(mainboard_HEIGHT * 0.2) 
                                        .build());

    let key_w = _WIDTH * 0.040;
    let cols = Columns::create()
                                .column(key_w).column(6.0) 
                                .column(key_w).column(6.0) 
                                .column(key_w).column(6.0) 
                                .column(key_w).column(6.0) 
                                .build();

    
    // First Row
    let first_row_keys: Vec<(&str,Option<&str>,i32,f64)> = vec![("NUM",Some("num_lock"),0,kw), ("/",Some("num/"),1,kw), ("*",Some("num*"),2,kw),("-",Some("num-"),3,kw)  ];
    let first_row = make_row_skinny(ctx, first_row_keys, cols.clone());
    
    // Second Row
    let second_row_keys: Vec<(&str,Option<&str>,i32,f64)> = vec![("7",Some("num7"),0,kw), ("8",Some("num8"),1,kw), ("9",Some("num9"),2,kw), ("+",Some("num+"),3,kw)  ];
    let second_row = make_row_skinny(ctx,second_row_keys,cols.clone());
    
    // third Row
    let third_row_keys: Vec<(&str,Option<&str>,i32,f64)> = vec![("4",Some("num4"),0,kw), ("5",Some("num5"),1,kw), ("6",Some("num6"),2,kw), ("+",Some("num+"),3,kw)  ];
    let third_row= make_row_skinny(ctx, third_row_keys,cols.clone());

    // Fourth Row
    let fourth_row_keys: Vec<(&str,Option<&str>,i32,f64)>= vec![("1",Some("num1"),0,kw), ("2",Some("num2"),1,kw), ("3",Some("num3"),2,kw), ("ENTER",Some("numenter"),3,kw)  ];
    let fourth_row = make_row_skinny(ctx,fourth_row_keys,cols.clone());
    
    // Fifth Row
    let fifth_row_keys: Vec<(&str,Option<&str>,i32,f64)> = vec![("INS",None,0,kw), (".",Some("num."),1,kw), ("ENTER",None,2,kw)  ];
    let cols2 = Columns::create()
                                .column(key_w*2.0 + 6.0).column(6.0) 
                                .column(key_w).column(6.0) 
                                .column(key_w).column(6.0) 
                                .build();
    let fifth_row = make_row_skinny(ctx,fifth_row_keys,cols2.clone());
    
    arrowkeys = arrowkeys.child(Container::create()
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

    return arrowkeys.build(ctx);
}





fn make_row_skinny(ctx: &mut BuildContext, row_keys: Vec<(&str,Option<&str>,i32,f64)> , cols:Columns ) -> Grid {
    let mut row_keys_grid =  Grid::create()
                                        .columns(cols);

    for i in row_keys.iter(){
        let (key,key_value ,pos,width) = i;
        let key_value:String = match key_value{
            None => {key.clone().to_string()}
            Some(x) => {x.to_string().clone()}
        }; 
        let fkey = generate_character_button(ctx,key.to_string(),key_value,*pos as usize * 2, *width);
        row_keys_grid = row_keys_grid.child(fkey);
    }
    row_keys_grid
}


fn make_row(ctx: &mut BuildContext, row_keys: Vec<(&str,Option<&str>,i32,f64)> ) -> Grid {
    let mut row_columns= Columns::create();
    for _ in 0..row_keys.len(){ row_columns=row_columns.column("auto").column(6.0); }

    let mut row =  Grid::create().columns(row_columns.build());
    for i in row_keys.iter(){
        let (key,key_value ,pos,width) = i;
        let key_value:String = match key_value{
            None => {key.clone().to_string()}
            Some(x) => {x.to_string().clone()}
        };
        let fkey = generate_character_button(ctx,key.to_string(),key_value,*pos as usize * 2, *width);
        row = row.child(fkey);
    }
    row
}



fn generate_character_button(
    ctx: &mut BuildContext,
    key: String,
    key_value: String,
    column: usize,
    width : f64,
) -> Entity {

    let mut button = Button::create()
        .class("single_content")
        .min_size(width, 48.0)
        .text(key.clone())
        .element("button")
        .margin((0.0, 8.0, 0.0, 0.0))
        .padding(20)
        .on_click(move | states, _| -> bool {
            print!("{}",key);
            change_key_colour(&key_value);
            // match key_value {
            //     None =>
            //     Some(x) =>
            //         {change_key_colour(&x);}
            // }
            // state(id, states).action(Action::Character(sight));
            true
        })
        .attach(Grid::column(column));

    if true {
        button = button.class("secondary");
        // button = button.class("primary");
    }
    button.build(ctx)
}

fn change_key_colour(key :&str){
    let white = "ffffff";
    // ./g810-led -dv 046d -dp c33c -tuk 1 -k '+key+' '+colour

    println!("Key: {}",key);

    let mut exitStatus = Command::new("sudo")
                                    .arg("./g810-led")
                                    .arg("-dv")
                                    .arg("046d")
                                    .arg("-dp")
                                    .arg("c33c")
                                    .arg("-tuk")
                                    .arg("-1")
                                    .arg("-k")
                                    .arg(key)
                                    .arg(white)
                                    .status()
                                    .expect("process failed to execute");
    println!("{}",exitStatus);

}

