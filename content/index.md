class: center
name: title
count: false

# Újraírtam Rustban..

.center[
.p60[
![Evangelism](https://external-preview.redd.it/Ikj0dtD2q1f70pJtxZEJahFAJH0LkkcdtNuxMWT8Dl0.jpg?auto=webp&s=1c6212f4d10bc678f00d19b36b99d0eba6a8ca79)
]
]

.me[.grey[**Leéh Péter**]]
.citation[`https://github.com/Ptrskay3/rust-workshop`]

---

name:intro

# Rust

---

template:intro

.lg[

- Relatíve fiatal, 1.0 verzió 2015-ben került kiadásra
  ]

---

template:intro

.lg[

- Relatíve fiatal, 1.0 verzió 2015-ben került kiadásra
- Open source, de a Rust Foundation mutatja az irányt
  ]

---

template:intro

.lg[

- Relatíve fiatal, 1.0 verzió 2015-ben került kiadásra
- Open source, de a Rust Foundation mutatja az irányt
- Fordított (machine code)
  ]

---

template:intro

.lg[

- Relatíve fiatal, 1.0 verzió 2015-ben került kiadásra
- Open source, de a Rust Foundation mutatja az irányt
- Fordított (machine code)
- Systems language (kompetitor a C és C++ mellett)
  ]

---

template:intro

.lg[

- Relatíve fiatal, 1.0 verzió 2015-ben került kiadásra
- Open source, de a Rust Foundation mutatja az irányt
- Fordított (machine code)
- Systems language (kompetitor a C és C++ mellett)
- Nincs GC és runtime
  ]

---

template:intro

.lg[

- Relatíve fiatal, 1.0 verzió 2015-ben került kiadásra
- Open source, de a Rust Foundation mutatja az irányt
- Fordított (machine code)
- Systems language (kompetitor a C és C++ mellett)
- Nincs GC és runtime
- Komplex statikus típus rendszer
  ]

---

template:intro

.lg[

- Relatíve fiatal, 1.0 verzió 2015-ben került kiadásra
- Open source, de a Rust Foundation mutatja az irányt
- Fordított (machine code)
- Systems language (kompetitor a C és C++ mellett)
- Nincs GC és runtime
- Komplex statikus típus rendszer
- Release 6 hetente (jelenleg 1.64.)
  ]

???

footnote

---

template:intro

.lg[

- Relatíve fiatal, 1.0 verzió 2015-ben került kiadásra
- Open source, de a Rust Foundation mutatja az irányt
- Fordított (machine code)
- Systems language (kompetitor a C és C++ mellett)
- Nincs GC és runtime
- Komplex statikus típus rendszer
- Release 6 hetente (jelenleg 1.64.)
- Stackoverflow survey: "Most loved language" 7. éve
  ]

???

footnote

---

template:intro

.lg[

- Relatíve fiatal, 1.0 verzió 2015-ben került kiadásra
- Open source, de a Rust Foundation mutatja az irányt
- Fordított (machine code)
- Systems language (kompetitor a C és C++ mellett)
- Nincs GC és runtime
- Komplex statikus típus rendszer
- Release 6 hetente (jelenleg 1.64.)
- Stackoverflow survey: "Most loved language" 7. éve

          ]

  .p80[.center[
  ![stackoverflow_rust_most_wanted](content/images/stackoverflow/rust_2022_want.png)
  ]]
  ???

footnote

---

# Rust

.center[
![promises](content/images/promises.png)
]

---

# Rust

- Modern nyelv
  - Generikus típusok, Trait-ek
  - Funkcionális programozási elemek: Enum, Pattern matching, Closure
  - Tooling
- Biztonság, a compiler és helyesség
- Zero cost FFI

---

#### Modern nyelv - Generikus típusok, Trait-ek

```rust
struct MyVector<T> {
  // ...
}

impl<T> MyVector<T> {
    pub fn find<C>(&self, condition: C) -> Option<&T>
    where
        C: Fn(&T) -> bool,
    {
        for v in self {
            if condition(v) {
                return Some(v);
            }
        }
        None
    }
}
```

---

#### Modern nyelv - Generikus típusok, Trait-ek

.moderate[

- Zero cost:
  - A compiler külön optimalizálja minden `T` és `C` típusra, amikkel hívásra kerül.
- `for v in self`:
  - Iterator trait - trait-ekhez hozzárendelhető egy bizonyos viselkedés.
- A Rust trait-ek nagyon hasonlóak más nyelvekbeli az interface-ekhez.
  ]

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl<T> Iterator for MyVector<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}
```

---

#### Modern nyelv - Generikus típusok, Trait-ek

.moderate[

- Traitek implementálhatóak bármilyen típusra, akár beépítettekre is.
- Csak akkor használható, ha az adott trait scope-ba importálva van.
  - ...vagyis nem szennyezi a namespace-t.
    ]

```rust
trait Hello {
  fn hello(&self);
}

impl Hello for bool {
  fn hello(&self) {
    println!("Hello!");
  }
}

fn main() {
  false.hello();
  // output: Hello!
}
```

---

#### Modern nyelv - Enum, Pattern matching, Closure

.moderate[

- Az `enum` egy centrális feature a nyelvben.
- Az enumok tartalmazhatnak valódi adatot, nem csak konstansokat.
  ]

```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust
if let Some(letter) = my_vector.find(|elem| elem.is_uppercase()) {
  // találtunk egy nagybetűt, letter néven elérhető
}

// vagy ekvivalens módon
match my_vector.find(|elem| elem.is_uppercase()) {
  Some(letter) => {
    // találtunk egy nagybetűt, letter néven elérhető
  }
  None => {
    // nincs nagybetű
  }
}
```

---

#### Modern nyelv - Enum, Pattern matching, Closure

```rust

enum GameEvent {
    PlayerLost,
    KeyPress(char),
    Click { x: f32, y:f32 },
}

let event = GameEvent::KeyPress('q');

match event {
    GameEvent::PlayerLost => println!("Player lost"),
    GameEvent::KeyPress('q' | 'Q') => println!("Q pressed"),
    GameEvent::KeyPress(_) => println!("Other key pressed"),
    GameEvent::Click { x, y } if x > 500. => println!("x is more than 500"),
    _ => println!("x was less than 500"),
}
```

---

#### Modern nyelv - Tooling

.moderate[

- Build tool és dependency manager: Cargo

  `Cargo.toml`:

```toml
[package]
version = "0.1.0"
# ...

[dependencies]
neon = { version = "0.9.1", optional = true, features = ["napi-6"] }
serde = "1.0"
converter_derive = { path = "../converter_derive" }
regex = { git = "https://github.com/rust-lang/regex", rev = "5197f21" }
```

- Formázás: Rustfmt

- Lint: Clippy - [Egy példa, ami nem rég hasznos volt számomra.](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=921f2441641cb7ac4830af1ccd7c75e8)

- IDE integráció: Rust-Analyzer, IntelliJ Rust plugin
  ]

---

#### Modern nyelv - Tooling

.moderate[

- Tesztelés:

```rust
#[test]
fn it_works() {
  assert_eq!(1 + 1, 2);
}
```

- Dokumentáció: Rust-doc, [docs.rs](https://docs.rs/tokio/latest/tokio/)
  ]

````rust
/// This is the documentation for the item below.
/// Code blocks are automatically treated and run as integration tests.
/// ```
/// assert_eq!(one_more(7), 8);
/// ```
fn one_more(n: i32) -> i32 {
  n + 1
}
````

---

#### Biztonság, a compiler és helyesség

.center[
![possible_errors](content/images/possible_errors.png)
]

---

#### Biztonság, a compiler és helyesség

- Alapértelmezetten minden változó immutable

```rust
let x = 5;
x += 1; // ❌
let mut y = 5;
y += 1; // ✅
```

- Egyedi koncepciók:
  - Ownership: [Minden változónak egy tulajdonosa van](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6a1416d377e9b6a85ac3df58e85c7d31)
  - Borrow checker
    - [Minden változóra teljesülnie kell, hogy egyidőben](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=482f77f3f0f7c523dfd6578bad01fe32)
      - bármennyi immutable referencia
      - csak egyetlen mutable referencia
  - [Thread-safety](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=58a3d153c6cc96f1ed4c1ca67d78661b)

---

name: it-works

#### Biztonság, a compiler és helyesség

- Explicit: nincsenek rejtett side-effektek, extra memória allokációk
- Nincsenek exception-ök, bármi ahol hiba történet a `Result` típust adja vissza.

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

```rust
// a num típusa itt Result<i32, ParseIntError>
let num: i32 = "42".parse();

// a ? operátor ekvivalens azzal, hogy hiba esetén térjen
// vissza az Err variánssal, egyébént pedig az Ok-kal
let num = "42".parse::<i32>()?;
```

---

template:it-works

.center[

- _"If it compiles, it works."_
  ]

---

#### Biztonság, a compiler és helyesség

.moderate[A hibaüzenetek érthetőek, néha már majdnem code review szintűek, sokszor a megoldást is tartalmazzák:]
.center[
![error2](content/images/errormsg1.jpeg)
]
.center[
![error3](content/images/errormsg4.jpg)
]

---

#### Biztonság, a compiler és helyesség

.p95[.center[
![error4](content/images/error_long.png)
]]

---

#### Makrók

-
- [serde](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e4535da77ddb16c0fed9ca4b38d05013)

---

#### Makrók

.p95[.center[
![sqlx_macro](content/images/sqlx_macro.gif)
]]

---

#### Async

- Teljesen támogatott 2019 óta
- Nincs beépített runtime
  - Standard, battle-tested: [tokio](https://tokio.rs)
- Async/await applikáció fejlesztésnél viszonylag egyszerű, library kódnál viszont komplex

#### Zero cost FFI

```rust
// C függvény hívása Rustból
extern "C" {
  fn abs(input: i32) -> i32;
}

fn main() {
  println!("{}", unsafe { abs(-10) });
}

// Rust függvény exportálása C-nek
#[no_mangle]
pub extern "C" fn rust_abs(input: i32) -> i32 {
  input.abs()
}
```

.moderate[

- Más nyelvekbe integrálhatóság
  - Node.js -> [Neon](https://neon-bindings.com/)
  - JavaScript -> [Wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
  - Python -> [PyO3](https://pyo3.rs/)
  - PHP -> [ext-php-rs](https://github.com/davidcole1340/ext-php-rs)
  - C++ -> [cxx](https://github.com/dtolnay/cxx)
  - Elixir -> [rustler](https://github.com/rusterlium/rustler)
  - .. [és még sok más.](https://areweextendingyet.github.io/)
    ]

---

#### Még alacsonyabb szintű kontroll

```rust
use std::arch::asm;

// Multiply x by 6 using shifts and adds
let mut x: u64 = 4;
unsafe {
    asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
    );
}
assert_eq!(x, 4 * 6);
```

---

#### Bár sokszor magas szintűnek érződik, de ...

```rust
impl<'w, 's, T: 'static> SystemParamFetch<'w, 's> for NonSendState<T> {
    type Item = NonSend<'w, T>;
    #[inline]
    unsafe fn get_param(
        state: &'s mut Self,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item {
        world.validate_non_send_access::<T>();
        let column = world
            .get_populated_resource_column(state.component_id)
            .unwrap_or_else(|| panic!("Non-send resource does not exist"));
        NonSend {
            value: &*column.get_data_ptr().cast::<T>().as_ptr(),
            ticks: column.get_ticks_unchecked(0).clone(),
            last_change_tick: system_meta.last_change_tick,
            change_tick,
        }
    }
}
```

---

name: lets-get-real

#### Rust bár sokszor magas szintűnek érződik, de ...

```
^ |
| |
P │ C++
e │
r │
f │
o │
r │
m │
a │
n │
c │
e │
  │                  JS, PHP, Python
  └─────────────────────────────────
Ease of use -->

```

???

OK, that's how Rust feels when it's working well. But let's be honest. It's not always like that.

---

template: lets-get-real

.bam-rust[
![Rust logo](content/images/rust-logo-512x512.png)
]

---

template: lets-get-real

.transparent[
.bam-rust[
![Rust logo](content/images/rust-logo-512x512.png)
]
]

.whimper-rust[
![Rust logo](content/images/rust-logo-512x512.png)
]

---

#### Ez csak a felszín egy része volt..

.center[
![iceberg](content/images/iceberg.webp)
]

---

name: drawbacks

### Hátrányok

.mid[

- Komplexebb, mint a legtöbb nyelv, nehezebb benne elérni a produktív szintet.
  ]

---

template:drawbacks
.mid[

- Rust nem OO.

  ]

---

template:drawbacks
.mid[

- Rust nem OO.

- Nincsenek előre buildelt library-k, többek között a generikus típusok miatt.

  - Mindig a forráskódból kell buildelni a library-t.
    ]

---

template:drawbacks
.mid[

- Rust nem OO.

- Nincsenek előre buildelt library-k, többek között a generikus típusok miatt.

  - Mindig a forráskódból kell buildelni a library-t.

- A compiler egy elég hosszú pipeline-t futtat, ezért lassú, de az elmúlt évben sokat fejlődött.
  ]

---

template:drawbacks
.mid[

- Rust nem OO.

- Nincsenek előre buildelt library-k, többek között a generikus típusok miatt.

  - Mindig a forráskódból kell buildelni a library-t.

- A compiler egy elég hosszú pipeline-t futtat, ezért lassú, de az elmúlt évben sokat fejlődött.

- Rust binary-k általában relatíve nagyobb méretűek

  - Főleg extra belefordított dolgok miatt, mint panic handler és debug szimbólumok,
    de van mód ezek kihagyására.

  ]

---

template:drawbacks
.mid[

- Rust nem OO.

- Nincsenek előre buildelt library-k, többek között a generikus típusok miatt.

  - Mindig a forráskódból kell buildelni a library-t.

- A compiler egy elég hosszú pipeline-t futtat, ezért lassú, de az elmúlt évben sokat fejlődött.

- Rust binary-k általában relatíve nagyobb méretűek

  - Főleg extra belefordított dolgok miatt, mint panic handler és debug szimbólumok,
    de van mód ezek kihagyására.

- Relatíve fiatal, így néhány library nem annyira kiforrott.
  ]

---

template:drawbacks
.mid[

- Rust nem OO.

- Nincsenek előre buildelt library-k, többek között a generikus típusok miatt.

  - Mindig a forráskódból kell buildelni a library-t.

- A compiler egy elég hosszú pipeline-t futtat, ezért lassú, de az elmúlt évben sokat fejlődött.

- Rust binary-k általában relatíve nagyobb méretűek (strip - Rust 1.59.)

  - Főleg extra belefordított dolgok miatt, mint panic handler és debug szimbólumok,
    de van mód ezek kihagyására.

- Relatíve fiatal, így néhány library nem annyira kiforrott.

- Az std minimális, de vannak "blessed" library-k, amiket Rust core team fejleszt.
  ]

---

### Hol használják?

.moderate[

- Beágyazott rendszerek, OS
- Blockchain
- Cloud infrastruktúra: [AWS Firecracker](https://firecracker-microvm.github.io/), [AWS Bottlerocket](https://aws.amazon.com/bottlerocket/)
- Játékfejlesztés: [Bevy](https://bevyengine.org/), [Amethyst](https://github.com/amethyst/amethyst)
- Backend: [Actix-web](https://actix.rs/), [Axum](https://github.com/tokio-rs/axum), [Rocket](https://rocket.rs/)
- Frontend: [Yew](https://yew.rs/), [Sycamore](https://sycamore-rs.netlify.app/), [Seed](https://seed-rs.org/)
- CLI: [Exa](https://github.com/ogham/exa), [Ripgrep](https://github.com/BurntSushi/ripgrep)
- Egyéb: [Tauri](https://github.com/tauri-apps/tauri), [MeiliSearch](https://github.com/meilisearch/MeiliSearch), [swc](https://github.com/swc-project/swc), [JetBrains Fleet](https://blog.jetbrains.com/fleet/2022/01/fleet-below-deck-part-i-architecture-overview/)
  ]

---

### Learning materials

- Bevezető:
  - ["The Book"](https://doc.rust-lang.org/book/)
  - [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
  - [Rustlings - Feladatok](https://github.com/rust-lang/rustlings/)
  - [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- Intermediate/Advanced
  - [Zero To Production in Rust](https://www.lpalmieri.com/) <-- Backend, Actix-Web
  - [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
  - [Writing an OS in Rust](https://os.phil-opp.com/)
  - [The Rustonomicon - Unsafe Rust](https://doc.rust-lang.org/stable/nomicon/)
  - [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)
  - [David Tolnay - Procedural Macro Workshop](https://github.com/dtolnay/proc-macro-workshop)
  - [Jon Gjengset - Főleg advanced Rust streamek](https://www.youtube.com/c/JonGjengset)
