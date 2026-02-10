use crate::game::begin;

pub mod game;

fn main() {
    begin();
    //let mut window = Window::new("My Window", start, update);
    //window.on_key_down(on_key_down);
    //window.on_mouse_button_down(mouse_callback);
    //window.on_mouse_motion(mouse_motion);
    //window.run();
}

/* 

pub fn mouse_motion(_commands: &mut Commands, _current_pos: Point) {
    // ...
}

pub fn mouse_callback(commands: &mut Commands, keycode: i32, cursor_pos: Point) {
    if cursor_pos.in_area(Point(0, 0), Point(100, 100)) && keycode == KeyCode::LMB {
        let mut counter = load!(commands, u32, "counter");
        *counter += 1;
        println!("counter: {}", *counter);
    }
}

pub fn on_key_down(commands: &mut Commands, keycode: i32) {
    let mut player = find!(commands, Actor, "Player");

    if keycode == KeyCode::UP {    
        player.transform.y -= 10;
    }
    if keycode == KeyCode::DOWN {
        player.transform.y += 10;
    }
    if keycode == KeyCode::RIGHT {
        player.transform.x += 10;
    }
    if keycode == KeyCode::LEFT {
        player.transform.x -= 10;
    }
}

pub fn start(commands: &mut Commands) {
    let player = make!(Actor::new("Player", "player"));
    commands.spawn(player.clone());
    
    let counter = 0u32;
    counter.make_global(commands);

} 

pub fn update(commands: &mut Commands) {}

*/