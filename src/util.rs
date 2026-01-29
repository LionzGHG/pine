
/// ## Description
/// The [KeyCode] struct provides constants that are associated to the representing [i32]-value for the
/// respective key.
/// ## Example
/// ```
/// fn key_callback(commands: &mut Commands, keycode: i32) {
///     if keycode == KeyCode::UP {
///         let player = commands.find("Player").unwrap();
///         qry!(player).transform.position.y += 10;
///     }
/// }
/// ```
pub struct KeyCode;

impl KeyCode {
    pub const BACK: i32 = 8;
    pub const ENTER: i32 = 13;
    pub const SHIFT: i32 = 16;
    pub const ESCAPE: i32 = 27;
    pub const SPACE: i32 = 32;

    pub const LEFT: i32 = 37;
    pub const UP: i32 = 38;
    pub const RIGHT: i32 = 39;
    pub const DOWN: i32 = 40;

    // TODO
}