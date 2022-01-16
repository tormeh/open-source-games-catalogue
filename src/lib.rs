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
    let initial_state = AppData {
        games: Arc::new(vec![veloren, zeroad]),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppData> {
    let list = Scroll::new(List::new(|| {
        Label::new(|game: &Game, _env: &_| game.name.clone())
            .align_vertical(UnitPoint::LEFT)
            .padding(10.0)
            .expand()
            .height(50.0)
            .background(Color::rgb(0.5, 0.5, 0.5))
        }))
        .vertical()
        .lens(AppData::games);


    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(list);

    // center the two widgets in the available space
    Align::centered(layout)
}