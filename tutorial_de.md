
# Feature-Übersicht (Auswahl) für die Pine Game-Engine - Einstieg

Jede Pine-Anwendung hat die gleiche Grundstruktur:

```rust
use pine::prelude::*;

fn main() {
    let mut window = Window::new_no_commands("Mein Fenster", start, update);
    window.start();
}

fn start() -> PineResult {
    // Wird einmal am Start des Programmes ausgeführt
    Ok(())
}

fn update() -> PineResult {
    // Wird jeden "tick" ausgeführt (bei 60 FPS = 60 mal die Sekunde)
    Ok(())
}
```

Wir erstellen ein Fenster mit `Window`. Durch Aufrufen der `start`-Methode öffnen wir das Fenster und beginnen
die Anwendung.

Die `start` und `update` Callbacks sind notwendig für jedes Programm. Darüberhinaus können wir für weitere Events
ebenfalls Callbacks definieren, so z.B. für Maus- und Tastatur-Eingaben:

```rust
fn main() {
    // --snip--
    window.on_key_down_no_commands(key_down);
    window.on_mouse_down_no_commands(mouse_down);
    // --snip--
}
```

`key_down` und `mouse_down` sind hierbei unsere Callback-Funktionen. Diese müssen wir definieren:

```rust
// `key_down` wird immer dann aufgerufen, wenn ein Tastatur-Key gedrückt wird 
fn key_down(keycode: i32) -> PineResult {

    Ok(())
}

// `mouse_down` wird immer dann ausgeführt, wenn ein Maus-Button gedrückt wird 
fn mouse_down(keycode: i32, pos: Point) -> PineResult {
    
    Ok(())
}
```

**Hinweis**: Falls die Fenstergröße von der Spielweltgröße abweichen soll, ist es möglich, die Spielweltgröße
relativ festzulegen:

```rust
window.set_logical_size(100, 100);
```
 
## Komponenten, Akteure und Attribute

Ein **Komponent** ist ein Spielobjekt, welches die Fähigkeit besitzt, auf den Bildschirm *gerendert* zu werden.
Eine besondere Komponente sind **Akteure**. Ein Akteur kann seinen Zustand - auf Benutzereingaben hin - dynamisch
verändern:

```rust
fn start() -> PineResult {
    let actor = make!(Actor::new("Mein Akteur", ""));
    Engine::spawn(actor)?;

    Ok(())
}
```

Erstellen wir einen neuen Akteur mit `Actor::new` ist dieser noch kein Komponent. Um ein Objekt zum Komponent zu machen,
müssen wir das `make!`-macro ausführen. Intern wird `Actor` dann in einen Pointer (`Rc<RefCell<dyn Component>>`) verpackt.

Um den Akteur tatsächlich auf den Bildschirm zu rendern nutzen wir die Funktion `Engine::spawn`. 

Das `Engine`-struct erlaubt es uns mit dem `Window` zu interagieren.

Wenn wir einen Akteur bearbeiten wollen, müssen wir ihn für einen bestimmten Scope aus seinem Pointer 'auspacken'. Dafür
verwenden wir die `Engine::capture` Funktion.

```rust
fn start() -> PineResult {
    let actor = make!(Actor::new("Mein Akteur", ""));

    // `actor` wird für den folgenden Scope als `a` 'eingefangen' (gecaptured):
    Engine::capture(actor.clone(), |a| {
        // a ist hier `&mut Actor`, statt `Rc<RefCell<dyn Component>>` und wir können ihn verändern, z.B.:
        a.set_size(vec2![100, 100]);
        a.set_color(Color::RED);
        a.set_position(Engine::get_world_center());
    }); // am Ende des scopes wird `a` gedropped

    // `actor` rendert nun als rotes 100x100 Quadrat in der Mite des Bildschirms
    Engine::spawn(actor)?;

    Ok(())
}
```

Wollen wir, dass ein Actor eine `Textur` erhält, müssen wir für den zweiten Parameter (welcher aktuell auf `""` gesetzt ist)
den Namen der Textur verwenden.

**Hinweis**: Die Textur muss im '.png'-Format im Ordner `./assets/textures/...` gespeichert sein!

**Attribute*** sind Fähigkeiten, welche Komponenten haben können. Beispiele für Attribute sind zum Beispiel `Collision2D` und
`Physics2D` für Kollision und Physik:

```rust
fn start() {
    let actor = make!(Actor::new("Spieler", "spieler_textur"));

    Engine::capture(actor.clone(), |a| {
        a.set_position(Engine::get_world_center());

        a.add_attribute(
            Collision2D::new("Spieler", a.get_size(), on_collision),
            "Spieler_Collision2D"
        );

        a.add_attribute(
            Physics2D::new("Spieler", Layer::GROUND),
            "Spieler_Physics2D"
        );
    });
}

// Wird aufgerufen, wenn `actor` mit einem anderen Actor `other` kollidiert, welcher ebenfalls das `Collision2D`
// Attribut hat
fn on_collision(other: &mut Actor) {}
```

**Komponenten** sowie **Attributen** kann der Anwender ebenfalls selbst definieren. Somit ist die Engine
flexibel erweiterbar.

Ein Komponent muss den `Component`- ein Attribut den `Attribute`-trait erweitern.

## Transform 

Jeder `Actor` besitzt ein `Transform`, mithilfe dessen wir seine Position in der Spielwelt verändern können:

```rust
// Aufbau des Transform-structs:
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub rotation: f32,
}
```

**Beispiel**: Simple Bewegung eines Actors auf Knopfdruck:

```rust
// --snip--
fn on_key(keycode: i32) -> PineResult {
    let spieler = Engine::get_actor("Spieler")?;

    Engine::capture(spieler, |s| {
        match keycode {
            KeyCode::UP     => s.transform.y -= 10.,
            KeyCode::DOWN   => s.transform.y += 10.,
            KeyCode::LEFT   => s.transform.x -= 10.,
            KeyCode::RIGHT  => s.transform.x += 10.,
            _               => (),
        }
    });

    Ok(())
}
// --snip--
```

## Tests

Pine verfügt über eine kleine Auswahl vorgefertigter Tests. Um diese auszuprobieren, gehe wie folgt vor:

```rust
use pine::prelude::*;

fn main() {
    // führt den `MinimalTest` aus
    Test::start::<MinimalTest>().verify();
}
```

Du kannst selbst `Test`s erstellen, indem du wie folgt vorgehst:

```rust
use pine::prelude::*;

pub struct MeinTest;

impl Testable for MeinTest {
    fn start() -> TestResult {
        // Verwende diese Methode wie die `main` Funktion bei normalen Anwendungen... 
        TestResult::SUCCESS
    }
}
```

## Globale Variablen

Über den internen Speicher der Engine können globale Variablen gespeichert werden, welche in den einzelnen
Teilfunktionen / Callbacks übergreifend verwendet werden können.

**Beispiel**: Eine globale boolean-Variable soll erstellt werden mit dem Namen "jumping" und dem Wert "false": 

```rust
let jumping = false;
Engine::add_global_var("jumping", &jumping)?;

// oder:
upload!("jumping", &jumping)?;

// oder:
false.make_global("jumping")?;
```

Nun soll diese Variable wieder geladen und verändert werden:

```rust
let jumping = Engine::get_global_var::<bool>("jumping")?;
*jumping = true;

// oder:
let jumping = load!("jumping", bool);
*jumping = true;
```

Die globale Variable soll nun gelöscht werden:

```rust
Engine::delete_global_var("jumping")?;
```

## Assets

`Assets` können über den **Asset-Browser** verwaltet werden. Alle Assets werden im Ordner `./assets/...` gespeichert.
Wir können Assets direkt laden, indem wir das `Assets`-struct verwenden, solange diese Assets im Assets-Ordner gespeichert
sind.

**Beispiel**: Ein Bild `spieler.png` ist im Ordner `./assets/textures` gespeichert und wir wollen es laden:

```rust
let texture = Assets::get::<Texture2D>("spieler.png")?;
```

Wir können ebenfalls `MediaFile`s (mp4-Datein) speichern und laden (für die `Video2D`-Komponente).

# Beispiel-Programm

```rust
use crate::tests::Testable;
use crate::prelude::*;

pub struct MovementTest;

impl Testable for MovementTest {
    fn run() -> super::TestResult {
        let mut window = Window::new_no_commands("Movement Test", start, update);
        window.set_logical_size(800, 600);
        window.on_key_down_no_commands(key_down);

        window.run();
        super::TestResult::SUCCESS
    }
}

fn key_down(keycode: i32) -> PineResult {
    let mut collision = load!("collision", bool);
    if *collision && keycode == KeyCode::SPACE {
        *load!("jumping", bool) = true;
        *collision = false;        
    }

    let actor = Engine::get_actor("Actor1")?;
    Engine::capture(actor, |a| {
        match keycode {
            KeyCode::LEFT   => a.transform.x -= 10.,
            KeyCode::RIGHT  => a.transform.x += 10.,
            _               => (), 
        }
    });

    Ok(())
}

fn on_collision(other: &mut Actor) {
    *load!("collision", bool) = true;
}

fn start() -> PineResult {
    let actor1 = make!(Actor::new("Actor1", "player"));

    false.make_global("collision")?;
    false.make_global("jumping")?;

    Engine::capture(actor1.clone(), |a| {
        a.set_size(vec2![100, 100]);
        a.set_position(Engine::get_world_center() - vec2![0, 100]);
        a.set_color(Color::GREEN);

        a.add_attribute(
            Collision2D::new("Actor1", a.get_size(), on_collision),
            "Actor1_Collision2D"
        );

        a.add_attribute(
            Physics2D::new("Actor1", Layer::GROUND),
            "Actor1_Physics2D"
        );
    });

    Engine::spawn(actor1)?;

    let actor2 = make!(Actor::new("Actor2", ""));

    Engine::capture(actor2.clone(), |a| {
        a.set_size(vec2![1000, 100]);
        a.set_position(Engine::get_world_center() + vec2![0, 100]);
        a.set_color(Color::BLUE);
        a.set_layer(Layer::GROUND);

        a.add_attribute(
            Collision2D::new_no_callback("Actor2", a.get_size()),
            "Actor2_Collision2D"
        );
    });

    Engine::spawn(actor2)?;

    Ok(())
}

fn update() -> PineResult {
    let actor = Engine::get_actor("Actor1")?;

    let mut jumping = load!("jumping", bool);

    Engine::capture(actor, |a| {
        let current_pos = a.transform.get_position();

        if *jumping {
            let target_pos = vec2![a.transform.x, Engine::get_world_center().y - 100.];
            let new_pos = Math::lerp_vec(current_pos, target_pos, 0.1);
            a.set_position(new_pos);

            if (new_pos - target_pos).length() < 50.0 {
                *jumping = false;
            }
        }
    });

    Ok(())
}
```
