//! Provides a procedural macro that easily generates a list
//! of assets, enums and types automating most of the
//! resource specification, loading, indexing and storage
//! in useful data structures.
//!
//! # Usage
//! Invoke the procedural macro `build_assets!`, surrounding a
//! description of your assets, folders and module/data structure.
//! Said description uses a DSL (*domain specific language*) with
//! a conventient, small grammar explained in the
//! [following section](#grammar). Additionally, an
//! [Example](#example) below to understand the syntax accepted by
//! `build_assets!`.
//!
//! <a name="grammar"></a>
//! # Grammar
//! The grammar is a rather small one, but it suffices to specify
//! structure, asset location and ordering.
//! ## Key
//! -   `::=` can be read as "is defined as".
//! -   `?` indicates 0 or 1 repetitions of the
//!     previous production.
//! -   `*` indicates 0 or more repetitions, and
//! -   `+` indicates 1 or more.
//! -   Parentheses are used to group expressions
//! -   `|` is used to offer a choice between two
//!     or more productions. None of them has
//!     priority over the other(s).
//! -   *Regular expressions* are used to decribe
//!     terminal symbols like `name`.
//! -   When terminals depend of external grammatical
//!     or semantic constraints, an angle-bracketed
//!     `<placeholder description>` is used.
//!
//! ```ebnf
//! document        ::= named-entry
//!
//! entry           ::= named-entry | content-entry
//! named-entry     ::= name filename
//! content-entry   ::= name content
//!
//! name            ::= [a-z][a-z0-9_]*
//! filename        ::= '"'<valid file name>'"'
//!
//! content         ::= ('/' '"'<valid folder name>'"')? '{' entry+ '}'
//! ```
//!
//! <a name="example"></a>
//! # Example
//! ```rust
//! build_assets!(
//!     res {
//!         image / "img/" {
//!             background "bgk.png"
//!             button {
//!                 up "up.png"
//!                 hover "hover.png"
//!                 down "down.png"
//!                 hidden "optional/hidden.png"
//!             }
//!             lever / "lever/" {
//!                 up "up.png"
//!                 down "down.png"
//!             }
//!         }
//!         font / "font" {
//!             noto_sans "NotoSans.woff"
//!         }
//!     }
//! );
//! ```
//!
//! The code above is simple, but it generates a *lot* of boilerplate
//! that otherwise would be extensive and rather time-consuming.
//!
//! Let's begin by describing the parts of the example:
//! -   `res` is an *item*. So are `image` and `background`. There
//!     are a few differences between them.
//!     -   `res` is a *structured* item, i.e., it has contents,
//!         which are specified between curly braces.
//!     -   Structured items may optionally specify a *sub-folder*
//!         for their files, by placing a slash `/` and a quoted
//!         folder name after the item name. `res` chose not to, but
//!         following our usual web structure, `image` did specify a
//!         folder for all image content.
//!
//!         Multiple directories in a hierarchy of structured
//!         items are simply concatenated and prepended to the
//!         files names inside.
//!     -   The outermost item **must** be a structured one.
//!     -   `background` is a *file* item, which simply specifies
//!         a file path afterwards. As stated early, paths in
//!         the surrounding hierarchy are prepended to it to
//!         resolve the final file name. For instance,
//!         the file in `hidden` inside `res/image/button`
//!         resolves to `img/optional/hidden.png`.
//!
//! From this code, the macro will generate `struct`s and
//! `enum`s that reflect the above structure. It will
//! produce the follwing Rust *items* (thought not necessarily
//! in this order):
//!
//! *   A module hierarchy
//!
//!     ```rust
//!     pub mod res {
//!         // ...
//!         pub mod image {
//!             // ...
//!         }
//!     }
//!     ```
//!     These modules will, of course, reflect the hierarchy
//!     defined in the macro. Note that only structured items
//!     that *contain other structured items* are given their
//!     own module/namespace. If a structured item contains
//!     only file items, it will not appear as a module.
//!
//! *   A series of `struct`s corresponding to asset containers,
//!     which use the modules to maintain hierarchy and all the
//!     benefits of proper name-spacing.
//!     
//!     ```rust
//!     pub mod res {
//!         pub mod image {
//!             pub struct Button {
//!                 pub up: HtmlImageElement,
//!                 pub hover: HtmlImageElement,
//!                 pub down: HtmlImageElement,
//!                 pub hidden: HtmlImageElement,
//!             }
//!
//!             pub struct Lever {
//!                 pub up: HtmlImageElement,
//!                 pub down: HtmlImageElement,
//!             }
//!         }
//!
//!         pub struct Image {
//!             pub background: HtmlImageElement,
//!             pub button: image::Button,
//!             pub lever: image::Lever,
//!         }
//!
//!         pub struc Font {
//!             pub noto_sans: String,
//!             // font-family name to use in SWIRL
//!         }
//!     }
//!
//!     pub struct Res {
//!         pub image: res::Image,
//!         pub font: res::Font,
//!     }
//!     ```
//!
//!     Notice that struct names result from using the
//!     names of structured items and transforming from
//!     `snake_case` to `PascalCase`.
//!
//! *   Additional code that allows a
//!     `swirl::loader::auto::Loader<Res>` just to
//!     be run, and (if loading succeeds) to return
//!     a populated `Res` instance instead of a dictionary.
//!
//!     ```rust
//!     use wasm_bindgen::{JsValue, throw_str};
//!     use web_sys::console;
//!     use swirl::loader::auto::Loader;
//!     use swirl::loader::macros::*;
//!     use swirl_asset::build_assets;
//!
//!     build_assets!(
//!         // ...
//!     );
//!
//!     let loader = Loader<Res>::new();
//!     loader.load(
//!         |res: Res| {
//!             console::info_1(&JsValue::from_str("all assets loaded"));
//!             run_the_thing(res);
//!         },
//!         |_| throw_str("Loading failed")
//!     );
//!     ```
//!
//! The resulting code, naturally, depends on components from
//! **`swirl`**. To import them into the current scope, just
//! add the line:
//!
//! ```rust
//! use swirl::loader::macros::*;
//! ```
//!
//! as in the previous code example.

//  A rather simple AST. 
mod ast {
    pub struct Entry {
        name    : String,
        content : Content,
    }
    
    pub enum Content {
        SingleFile(String),
        Components(Vec<Entry>),
    }
}


