// Example 0: The Window
// The simplest example: Do absolutely nothing other than just opening a window

use mint::Vector2;
use quicksilver::{
    graphics::Graphics,
    lifecycle::{run, EventStream, Settings, Window},
    Result,
};

// main() serves as our kicking-off point, but it doesn't have our application logic
// Actual logic goes in our app function, which is async
// 'run' manages loading resources asynchronously and dealing with the event loop
fn main() {
    run(
        Settings {
            size: Vector2 { x: 800.0, y: 600.0 },
            title: "Window Example",
            ..Settings::default()
        },
        app,
    );
}

// Our actual logic! Not much to see for this example
async fn app(_window: Window, _gfx: Graphics, mut events: EventStream) -> Result<()> {
    loop {
        while let Some(_) = events.next_event().await {
            // Normally we'd do some processing here
        }
        // And then we'd do updates and drawing here
        // When this loop ends, the window will close and the application will stop
        // If the window is closed, our application will receive a close event and terminate also
    }
}
