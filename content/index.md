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
- StackOverflow survey: "Most loved language" 7. éve
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
- StackOverflow survey: "Most loved language" 7. éve

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

# Rust - recent articles

[Programming languages endorsed for server-side use at Meta](https://engineering.fb.com/2022/07/27/developer-tools/programming-languages-endorsed-for-server-side-use-at-meta/)

> - For performance-sensitive back-end services, we encourage C++ and Rust. Rust is a new addition to this list. There’s a rapidly increasing Rust footprint in our products and services, and we’re committing to Rust long-term and welcome early adopters.

> - For CLI tools, we recommend Rust. This is a new recommendation for this year.

---

# Rust - recent articles

- Linux: [Linus Torvalds: Rust will go into Linux 6.1](https://www.zdnet.com/article/linus-torvalds-rust-will-go-into-linux-6-1/)
- AWS: [Sustainability with Rust](https://aws.amazon.com/blogs/opensource/sustainability-with-rust/)
- Google: [Rust in the Android platform](https://security.googleblog.com/2021/04/rust-in-android-platform.html), [How Android is using Rust](https://www.youtube.com/watch?v=SU8clrSVWtI)
- Microsoft: [Announcing Rust for Windows v0.9](https://blogs.windows.com/windowsdeveloper/2021/05/06/announcing-rust-for-windows-v0-9/), [Rust Is the Industry’s ‘Best Chance’ at Safe Systems Programming](https://thenewstack.io/microsoft-rust-is-the-industrys-best-chance-at-safe-systems-programming/)
- Discord: [Why Discord is switching from Go to Rust](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)
- Volvo: [Why Rust is actually good for your car](https://medium.com/volvo-cars-engineering/why-volvo-thinks-you-should-have-rust-in-your-car-4320bd639e09)
- Cloudera: [How we built Pingora, the proxy that connects Cloudflare to the Internet](https://blog.cloudflare.com/how-we-built-pingora-the-proxy-that-connects-cloudflare-to-the-internet/)

---

#### Egyedi koncepciók, biztonság, compiler és helyesség

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
match my_vector.find(|elem| elem.is_uppercase()) {
  Some(letter) => {
    // találtunk egy nagybetűt, letter néven elérhető
  }
  None => {
    // nincs nagybetű
  }
}

// vagy ekvivalens módon
if let Some(letter) = my_vector.find(|elem| elem.is_uppercase()) {
  // találtunk egy nagybetűt, letter néven elérhető
}

```

---

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
let num = "42".parse::<i32>();

// a ? operátor (nagyjából) ekvivalens azzal, hogy hiba esetén térjen
// vissza az Err variánssal, egyébént pedig az Ok-kal
let num = "42".parse::<i32>()?;
```

---

#### Biztonság, a compiler és helyesség

```rust
fn get_user_id(session: Session) -> Result<Uuid, ApiError> {
  session
    .get::<Uuid>("user_id")
    .ok_or(ApiError::Unauthorized)?
}
```

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

- Traitek implementálhatóak bármilyen típusra, akár beépítettekre is. (Orphan-rule)
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

```rust
enum GameEvent {
    PlayerLost,
    KeyPress(char),
    Click { x: f32, y:f32 },
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

#### Modern nyelv - Enum, Pattern matching, Closure

Minden variánst kezelni kell

```rust
match event {
    GameEvent::PlayerLost => println!("Player lost"),
    GameEvent::KeyPress('q' | 'Q') => println!("Q pressed"),
    GameEvent::KeyPress(_) => println!("Other key pressed"),
    GameEvent::Click { x, y } if x > 500. => println!("x is more than 500"),
}
```

---

#### Modern nyelv - Enum, Pattern matching, Closure

Minden variánst kezelni kell

```rust
match event {
    GameEvent::PlayerLost => println!("Player lost"),
    GameEvent::KeyPress('q' | 'Q') => println!("Q pressed"),
    GameEvent::KeyPress(_) => println!("Other key pressed"),
    GameEvent::Click { x, y } if x > 500. => println!("x is more than 500"),
}
```

.center[
![match_exhaustive](content/images/match_exhaustive_light.png)
]

---

#### Biztonság, a compiler és helyesség - TODO: misplaced slide

<br><br/>
<br><br/>

```ts
textObject.lineHeight =
  ((parseFloat(paragraphStyles[0]?.lineSpacing) *
    mozaBook.defaultLineSpacing[parsedText.textFamily]) /
    maxFontSize || 1.0) * lineSpacingRatio;
```

---

#### Biztonság, a compiler és helyesség

.moderate[A hibaüzenetek érthetőek, néha már majdnem code review szintűek, sokszor a megoldást is tartalmazzák:]
.p85[.center[
![error_num](content/images/error_num_light.png)
]
]

---

#### Biztonság, a compiler és helyesség

.center[
![error_comp](content/images/error_comp_light.png)
]
.p85[
.center[
![error_greek](content/images/error_greek_light.png)
]]

---

#### Biztonság, a compiler és helyesség

<br><br/>

.p95[.center[
![sleep](content/images/sleep.png)
]]

---

#### Biztonság, a compiler és helyesség

<br><br/>
.p95[.center[
![sleep](content/images/sleep.png)
]]

.center[

(jk)

]

---

#### Biztonság, a compiler és helyesség

.p95[.center[
![crate_1](content/images/crates_long1.png)
]]

.p95[.center[
![crate_2](content/images/crates_long2.png)
]]

.p95[.center[
![crate_3](content/images/crates_long3.png)
]]

---

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

- Language server: Rust-Analyzer
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

#### Makrók

<br><br/>

- "Kód ami kódot ír/transzformál"
- Fordítás-időben az AST-n hajtódik végre
- Kevesebb boilerplate + extra funkcionalitás
- `println!`
- Általában komplex implementálni

---

#### Makrók

<br><br/>

- "Kód ami kódot ír/transzformál"
- Fordítás-időben az AST-n hajtódik végre
- Kevesebb boilerplate + extra funkcionalitás
- `println!`
- Általában komplex implementálni (nagyon)

---

#### Makrók

<br><br/>

.p95[.center[
![sqlx_macro](content/images/sqlx_macro.gif)
]]

---

#### Makrók

```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

---

#### Async # FIXME: talk about this in the previous slide, and delete?

- Teljesen támogatott 2019 óta
- Nincs beépített runtime
  - Standard, battle-tested: [tokio](https://tokio.rs)
- Async/await applikáció fejlesztésnél viszonylag egyszerű, library kódnál viszont komplex
  TODO: code?

```rust
async fn say_hello() {
  println!("Hello");
}

#[tokio::main]
async fn main() {
  say_hello().await;
}
```

---

#### WebAssembly

<br/><br/>

- Új Assembly-szerű nyelv amit a böngésző végre tud hajtani megközelítőleg natív sebességgel
- Már nem csak böngésző, van egyedülálló runtime
  - Cloud megoldások ([wasmcloud](https://wasmcloud.com/), [wasmedge](https://wasmedge.org/))
  - hasonló az AWS Lambda-hoz
- Rust fordítható WebAssembly targetre

---

#### WebAssembly - frontend

```rust
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let state = use_state(|| 0);

    let increment_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    html! {
        <>
            <p> {"current count: "} {*state} </p>
            <button onclick={increment_counter}>{"+"}</button>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

---

#### WebAssembly - frontend

<br/><br/>
<br/><br/>
.center[
![yew_render](content/images/yew.gif)
]

---

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
  - Node.js -> [Neon](https://neon-bindings.com/), [N-API](https://napi.rs/)
  - JavaScript -> [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
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
fn main() {
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

#### Roadmap

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

- Rust binary-k általában relatíve nagyobb méretűek (strip since 1.59.)

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

- Rust binary-k általában relatíve nagyobb méretűek (strip since 1.59.)

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

- Rust binary-k általában relatíve nagyobb méretűek (strip since 1.59.)

  - Főleg extra belefordított dolgok miatt, mint panic handler és debug szimbólumok,
    de van mód ezek kihagyására.

- Relatíve fiatal, így néhány library nem annyira kiforrott.

- Az std minimális, de vannak "blessed" library-k, amiket Rust core team fejleszt.
  ]

---

### Hol használják?

.moderate[

- Beágyazott rendszerek, OS: [Redox OS](https://www.redox-os.org/)
- Blockchain: [Solana](https://solana.com/)
- Cloud infrastruktúra: [AWS Firecracker](https://firecracker-microvm.github.io/), [AWS Bottlerocket](https://aws.amazon.com/bottlerocket/)
- Adatbázis: [SurrealDB](https://surrealdb.com/)
- Játékfejlesztés: [Bevy](https://bevyengine.org/), [Amethyst](https://github.com/amethyst/amethyst)
- Back-end: [Actix-web](https://actix.rs/), [Axum](https://github.com/tokio-rs/axum), [Rocket](https://rocket.rs/)
- Front-end: [Yew](https://yew.rs/), [Sycamore](https://sycamore-rs.netlify.app/), [Dioxus](https://dioxuslabs.com/)
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
  - [Zero To Production in Rust](https://www.lpalmieri.com/)
  - [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
  - [Writing an OS in Rust](https://os.phil-opp.com/)
  - [The Rustonomicon - Unsafe Rust](https://doc.rust-lang.org/stable/nomicon/)
  - [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)
  - [David Tolnay - Procedural Macro Workshop](https://github.com/dtolnay/proc-macro-workshop)
  - [Jon Gjengset - Advanced Rust streams](https://www.youtube.com/c/JonGjengset)
