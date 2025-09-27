# The SADL Language and Compiler

The compiler (`sadlc`) translates **SWIRL**'s simple *assets-description
language* file (`.sadl`) into **Rust** files. The generated code takes care of:

- Generating *hierarchical* `enum`erations and `struct`ures. The structures can
  be queried through members or indexed through the various `enum`s.

- Generating code to load all assets, with a top-level `struct` as result.

- Generating an initial scene graph

For the time being, SADL is only aware of *font*, *audio* and *image* formats.

- Fonts should be `.woff` files, as this format is universally recognized by all
  major browsers.

- When provided a single audio file with the extension `.mp3` or `.ogg`, it will
  automatically include a file with the other extension as an alternative.
  
- Images are currently limited to `.jpg` and `.png` types. It is possible to
  specify a numerically-tagged *range* of images, which the compiler will use to
  create a `SpriteMovie`.

This project began as a Rust *procedural macro*; however, letting users see the
generated code was deemed necessary for optimal usability and easier learning.
Users can inspect it and *see* types and declarations instead of having to
remember names or derive them through the compiler's rules every time.


## Building

See [BUILDING.md](./BUILDING.md) for compiling instructions.


## Syntax

The syntax of `.sadl` files is rather simple. Spaces, tabs and newlines are
ignored; the language is *free-form*, letting users choose whichever formatting
convention they prefer. It is, however, **case-sensitive**.

Line comments in Rust-style (`//`) and Haskell-style (`--`) are both allowed.
They're ignored, and absent from the compiler's output.

Every file is divided into two main sections: `assets` and `graph`. To begin
a section, just write the section name followed by a colon and a function name:

    assets: loader_fn

The `assets` section is mandatory; `graph` is optional.


### The `assets` Section

The assets section is a hierarchical description of files to be loaded and
the function that does so:

    assets: loader_fn
    
        OuterGroup {
            SimpleElement = "filepath.jpg",
            RangedElement = "prefix" + [1 ..= 25] + "postfix.jpg",
            GroupElement {
                InnerSimpleElement1 = "another-filepath1.jpg",
                InnerRangedElement = "another-prefix-" + [ .. 1000 : 3] + ".png"
                InnerSimpleElement2 = "another-filepath2.woff",
                InnerGroupElement {
                    InnermostSimpleElement1 = "whatever.mp3",
                },
            },
        }

First, it specifies the loading function's name using a *value* name, i.e., a
name befitting a variable or function. Value names must start with a lower-case
letter and can be followed with more letters (lower- or upper-cased), digits and
underscores. This is so the syntax for multiple languages is covered; when
output is generated, the target language's convention is respected. This means
that, had we used Haskell's convention (i.e., *lowerCamelCase*):

    assets: loaderFn

the output would convert every lower/upper pair into the *snake_case*
equivalent, still yielding `loader_fn`.

What follows is a hierarchy of *named* resources in resource categories. Those
categories (or **groups**, as we call it in SADL) can contain several
**elements**, each representing a file or set of files, or other groups for
further organizational division. Group and element names must be in
*CamelCase*, (aka *PascalCase*) following both Haskell and Rust's
conventions for data constructors.

In the target files, groups are realized as `struct`s and `enum`s, (appending,
respectively, `Struct` and `Set` to their original names.) One-file values
become variants/contructors for those `enum`s -- retaining their exact name --
and `struct` fields, converting the name to *snake_case*.  The `struct`
hierarchy will be used to load and contain the actual resources, and the `enum`s
will be used as indexers to the former.

Besides `struct`s and `enum`s, the `assets` description also allows the compiler
to output a loader function, which has the following signature:

    pub fn loader_fn(
        result: Rc<RefCell<Result<OuterGroupStruct, JsValue>>>
    ) -> Promise

The `result` parameter is a consequence of **js-sys**`s `Promises` being unable
to handle anything other than `JsValue`s. The `Promise` will be always
*resolved* without value; the `result` needs to be read afterwards to determine
the call's resolution.

There are few rules regarding the resource hierarchy:

- The loader function, just like the `assets` section, is mandatory.

- All names in the hierarchy must be in *PascalCase* (aka *CamelCase*.)
  Only letters and digits may be used, and every name must begin with an
  uppercase letter.  Generally, every consecutive word is supposed to start
  with an uppercase letter as well, but the compiler doesn't care.

- There must be a single topmost group (in this example, `OuterGroupName`.)

- Names must **not** end in `Struct` or `Set`. These are appended to group
  names for enumeration and record names.

- Name duplication is forbidden, even at different levels.

*Groups* are made up of elements and other groups (*nesting* is allowed -- and
encouraged.) *Simple* elements refer to a single file (unless it's a *sound*
file, in which case it references two.) *Range* elements refer to a list of
files using identical prefixes and postfixes, but with a different numbers
between those. These are all described in more detail below.


#### Simple elements

The simplest item definitions are those followed by an equal sign and a **valid**
filename (it must have a loadable extension, such as `.jpg` or `.woff`.)

    SimpleElement = "filepath.jpg"

If the extension of the filename happens to be `.ogg` or `.mp3`, it will be
identified as a sound and generate code to load a sound trying both extensions,
allowing the browser to pick the one it supports the best.

The name of the variant would be the same, as part of the `OuterGroupSet`
enumeration as per our example. The *field* name would be the *snake_cased*
version, i.e., `simple_element` (all lowercase, with underscores between word
limits), which in this case would be part of a `OuterGroupStruct`.

File names (and parts of names) are double-quoted, and there are restrictions on
valid characters -- namely, any *printable* character can be used except for the
following:

    < > # % @ $ & + * ? ! , ; : / \ | [ ] { } ' " `

The rule attempts to prevent the resulting URLs from transcription errors or
file system rejection.


#### Ranged elements

These specify a range and optional zero-padding.  These are reserved for image
sequences, and will be converted into `SpriteMovie`s. Both **Rust** range operators
(`..`, `..=`) can be used to specify a range. Example:

    RangedElement = "prefix" + [1 ..= 25] + "postfix.jpg"
    
This will load a sequence of files named `prefix1postfix.jpg`  through  
`prefix25postfix.jpg`. In this case, the last number is included in the range by
the `..=` operator.

For zero-padded numbering, simply add a colon and the total number of digits:

    InnerRangedElement = "another-prefix" [ .. 1000 : 3] ".png"

As shown above, the initial number can be omitted and assumed to be zero. The
resulting files would go from `another-prefix-000.png` to
`another-prefix-999.png`. The `..` operator excludes the upper limit.

Regarding generated code, `RangedElement` would keep its name as a variant,
and become field `ranged_element` for its immediate parent, `OuterGroupStruct`.
It would also generate its own enumeration and record types, `RangedElementSet`
and `RangedElementStruct`. The set would contain variants `RangedElement1`
through `RangedElement25` (zero-padding or absence thereof would be respected
here as well), and the record would have fields `ranged_element1` to
`ranged_element25`.


#### Complex elements (aka Group Elements)

These elements are groups that contain further elements. The leading identifier
is followed by an opening brace (`{`), a list of definitions (simple, ranged or
complex) separated by commas, and the closing brace (`}` .)

    GroupElement {
        InnerSimpleElement1 = "another-filepath1.jpg",
        InnerRangedElement = "another-prefix-" + [ .. 1000 : 3] + ".png"
        InnerSimpleElement2 = "another-filepath2.woff",
        InnerGroupElement {
            InnermostSimpleElement1 = "whatever.mp3",
        },
    },

In a very Rust-like way, the last definition inside a group may be followed by
a trailing comma. There's no pre-established limit to the number of groups that
can be nested inside one another.


### The `graph` Section

This optional section describes the *scene graph*, i.e., the visual hierarchy of
the scene to be rendered, which is necessarily a **Directed Acyclic Graph**.  If
used, it begins with the `graph :` header followed by a scene-constructing
function name, an arrow and the desired name for its return type:

    graph: mise_en_scene -> Scene

The function name -- like the one used in the `assets` section -- has to start
with a lower-case letter and be *snake_cased*, *lowerCamelCased*, or a
combination of both, and it will be normalized to the target language's naming
conventions as well. The generated function will have the following signature:

    pub fn mise_en_scene(assets: &OuterGroupStruct) -> Result<Scene, JsValue>


