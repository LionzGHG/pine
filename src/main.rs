use pine::prelude::*;

fn main() {
    let mut window = Window::new("My Window", start, update);
    window.on_key_down(on_key_down);
    window.run();
}

pub fn on_key_down(_commands: &mut Commands, keycode: i32) {
    if keycode == KeyCode::SPACE {
        println!("space pressed!");
    }
}

pub fn start(commands: &mut Commands) {
    let label = make!(Label::new("Label1", "Hello, World!", 100, 100));
    commands.spawn(cpy!(label));

    // let texture = Assets::get::<Texture2D>("player").unwrap();
    // let player = make!(Actor::new("Player", texture));
    // commands.spawn(cpy!(player));

    qry!(label).text = "Hello".to_string();
} 

pub fn update(commands: &mut Commands) {
    let label = commands.find("Label1").unwrap();
}