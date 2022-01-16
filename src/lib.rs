use druid::widget::{Align, Flex, Label, Scroll, List};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc, Color, UnitPoint};
use std::sync::Arc;

use wasm_bindgen::prelude::*;

#[derive(Clone, Data, Lens)]
struct AppData {
    games: Arc<Vec<Game>>,
}

#[derive(Clone, Data, Lens)]
struct Game {
    name: String,
    language: String
}

// This wrapper function is the primary modification we're making to the vanilla
// hello.rs example.
#[wasm_bindgen]
pub fn wasm_main() {
    // This hook is necessary to get panic messages in the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main()
}

pub fn main() {
    // describe the main window
    //
    // Window title is set in index.html and window size is ignored on the web,
    // so can we leave those off.
    let main_window = WindowDesc::new(build_root_widget());

    // create the initial app state
    let veloren = Game{name:"Veloren".to_string(), language:"Rust".to_string()};
    let zeroad = Game{name:"0ad".to_string(), language:"C++".to_string()};
    let stk = Game{name:"Super Tux Kart".to_string(), language:"C++".to_string()};
    let initial_state = AppData {
        games: Arc::new(vec![veloren, zeroad, stk]),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppData> {
    let list = Scroll::new(List::new(|| {
        let name = Label::new(|game: &Game, _env: &_| game.name.clone());
        let language = Label::new(|game: &Game, _env: &_| game.language.clone());
        Flex::row()
            .with_child(name)
            .with_flex_spacer(10.0)
            .with_child(language)
            .padding(10.0)
            .background(Color::rgb(0.1, 0.1, 0.1))
        }))
        .vertical()
        .lens(AppData::games);




    // center the two widgets in the available space
    Align::centered(list)
}