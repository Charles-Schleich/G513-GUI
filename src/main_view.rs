use orbtk::prelude::*;

use std::process::Command;
use crate::MainState;

widget!(
    MainView<MainState> {
        title: String16
    }
);

const _HEIGHT:f64 = 500.0;
const _WIDTH:f64 = 1000.0;
// const _FIRST_ROW: [ (&str,i8) ; 14 ]  = [ ("~",0) ,("1",1) ,("2",2) ,("3",3) ,("4",4) ,("5",5) ,("6",6) ,("7",7) ,("8",8) ,("9",9) ,("0",10) ,("-",11) ,("=",12) ,("BKSPC",13) ];



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

fn make_f_keys(ctx: &mut BuildContext) -> Entity {
    let kw = 48.0 ; //  Standard Key width
    
    let cols = Columns::create()
                                .column(_WIDTH * 0.050 ).column(6.0)// ESC
                                .column(_WIDTH * 0.0315 ).column(6.0)// ___
                                .column(_WIDTH * 0.050 ).column(6.0)// F1
                                .column(_WIDTH * 0.050 ).column(6.0)
                                .column(_WIDTH * 0.050 ).column(6.0)
                                .column(_WIDTH * 0.050 ).column(6.0)
                                .column(_WIDTH * 0.035 ).column(6.0) // __
                                .column(_WIDTH * 0.050 ).column(6.0)//F5
                                .column(_WIDTH * 0.050 ).column(6.0)
                                .column(_WIDTH * 0.050 ).column(6.0)
                                .column(_WIDTH * 0.050 ).column(6.0)
                                .column(_WIDTH * 0.035 ).column(6.0)
                                .column(_WIDTH * 0.050 ).column(6.0) //F9
                                .column(_WIDTH * 0.050 ).column(6.0)
                                .column(_WIDTH * 0.050 ).column(6.0)
                                .column(_WIDTH * 0.050 ).column(6.0) //F12
                                .build();


    let mut fkeys =  Grid::create()
                            .columns(cols);

    let esc = generate_character_button(ctx,"ESC".to_string(),0,kw);
    fkeys = fkeys.child(esc);
    let fkey_letters  = [("F1",2,kw),("F2",3,kw),("F3",4,kw),("F4",5,kw),("F5",7,kw),("F6",8,kw),("F7",9,kw),("F8",10,kw),("F9",12,kw),("F10",13,kw),("F11",14,kw),("F12",15,kw)];

    for i in fkey_letters.iter(){
        let (key,pos,width ) = i;
        // let fkey = generate_text(ctx,key.to_string(),*pos as usize);
        let fkey = generate_character_button(ctx, key.to_string(),*pos as usize * 2 , *width);
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
    
    let first_row_keys: Vec<(&str,i32,f64)>  = vec![("~",0,kw) ,("1",1,kw) ,("2",2,kw) ,("3",3,kw) ,("4",4,kw) ,("5",5,kw) ,("6",6,kw) ,("7",7,kw) ,("8",8,kw) ,("9",9,kw) ,("0",10,kw) ,("-",11,kw) ,("=",12,kw) ,("BKSPC",13,115.0) ];
    let first_row= make_row(ctx, first_row_keys);

    // // Second Row
    let second_row_keys: Vec<(&str,i32,f64)>  = vec![("TAB",0,66.66) ,("Q",1,kw) ,("W",2,kw) ,("E",3,kw) ,("R",4,kw) ,("T",5,kw) ,("Y",6,kw) ,("U",7,kw) ,("I",8,kw) ,("O",9,kw) ,("P",10,kw) ,("[",11,kw) ,("]",12,kw) ,("ENTER",13,70.0) ];
    let second_row= make_row(ctx, second_row_keys);

    // // third Row
    let third_row_keys: Vec<(&str,i32,f64)>  = vec![("CAPS",0,120.0) ,("A",1,kw) ,("S",2,kw) ,("D",3,kw) ,("F",4,kw) ,("G",5,kw) ,("H",6,kw) ,("J",7,kw) ,("K",8,kw) ,("L",9,kw) ,(":",10,kw) ,("\"",11,kw) ,("\\",12,kw)];
    let third_row= make_row(ctx, third_row_keys);

    // // Fourth Row
    let fourth_row_keys: Vec<(&str,i32,f64)>  = vec![("LSHIFT",0,140.0) ,("Z",1,kw) ,("X",2,kw) ,("C",3,kw) ,("V",4,kw) ,("B",5,kw) ,("N",6 ,kw) ,("M",7 ,kw) ,("<",8 ,kw) ,(">",9 ,kw) ,("?",10 ,kw) ,("RSHIFT",11 ,130.67) ];
    let fourth_row= make_row(ctx, fourth_row_keys);

    // // Fifth Row
    let bk = 56.0;
    let fifth_row_keys: Vec<(&str,i32,f64)>  = vec![("LCTRL",0,66.66) ,("WIN",1,bk) ,("ALT",2,bk) ,("SPACE",3,270.0) ,("ALTGR",4,bk) ,("FN",5,bk) ,("OPT",6 ,bk) ,("RCTRL",7 ,66.66)];
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

fn make_row(ctx: &mut BuildContext, row_keys: Vec<(&str,i32,f64)> ) -> Grid {
    let key_w = _WIDTH * 0.0405;
    let mut row_columns= Columns::create();
    for _ in 0..row_keys.len(){ row_columns=row_columns.column("auto").column(6.0); }

    let mut row =  Grid::create().columns(row_columns.build());
    for i in row_keys.iter(){
        let (key,pos,width) = i;
        let fkey = generate_character_button(ctx,key.to_string(),*pos as usize * 2, *width);
        row = row.child(fkey);
    }
    row
}

fn generate_character_button(
    ctx: &mut BuildContext,
    key: String,
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
        // .vertical_alignment()
        .on_click(move | states, _| -> bool {
            print!("{}",key);
            change_key_colour(&key);
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

    let _key = match key {
        "BKSPC"=> "f", 
        "LCTRL" => "ctrll",
        "RCTRL" => "ctrlright",
        "ALT" => "altright",
        "LSHIFT" => "shiftleft",
        "RSHIFT" => "shiftright",
        "CAPS" => "capslock",
        ":"=>"semicolon",
        "\'"=>"quote",
        "WIN" => "winleft",
        n => {n}
    };

    let mut exitStatus = Command::new("sudo")
                                    .arg("./g810-led")
                                    .arg("-dv")
                                    .arg("046d")
                                    .arg("-dp")
                                    .arg("c33c")
                                    .arg("-tuk")
                                    .arg("-1")
                                    .arg("-k")
                                    .arg(_key)
                                    .arg(white)
                                    .status()
                                    .expect("process failed to execute");
    println!("{}",exitStatus);

}

