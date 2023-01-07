<h1 align="center"><img src="assets/logo.svg" height="80px" alt="Frui"/></h1>

*<p align="center">Reading: "Fru–" as in "fruit" and "–i" as in "I am".</p>*

<p align="center">
<a href="https://crates.io/crates/frui"><img src="https://img.shields.io/crates/v/frui.svg" alt="Latest version"/></a>
<a href="https://github.com/fruiframework/frui/blob/main/LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT"/></a>
<a href="https://github.com/fruiframework/frui/blob/main/LICENSE-APACHE"><img src="https://img.shields.io/badge/license-Apache-blue.svg" alt="Apache"/></a>
<a href="https://discord.gg/CCVygCAgXR"><img src="https://img.shields.io/badge/Join-flat.svg?label=Discord&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2" alt="Discord"/></a>
</p>

## What is Frui?

Frui is a developer-friendly UI framework that makes building user interfaces easy and productive. It's inspired by Flutter architecture and is written in Rust!

For an introduction see the [announcement](https://github.com/fruiframework/frui-blog/blob/master/posts/001/0.0.1.md).

## Example

```rust
#![feature(type_alias_impl_trait)]

use frui::prelude::*;

#[derive(ViewWidget)]
struct App<'a>(&'a str);

impl ViewWidget for App<'_> {
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Self::Widget<'w> {
        Center::child(Text::new(format!("Hello, {}!", self.0)))
    }
}

fn main() {
    run_app(App("World"));
}
```

## Warning

Frui is an experimental framework and is not suitable for building real applications. It is more of a proof of concept and an exploration of new ideas, rather than a fully-fledged and reliable tool. While it may have some interesting implications and possibilities, it is not yet ready for production use and the development efforts on it are rather sporadic. It is currently not optimized and some important features have not been implemented. 

To compile it you will need to install the **latest version of nightly Rust**. 

Feel free to try it out!

## Features

*Ok, what's in there?*

- Basic widgets:
  - `ViewWidget`
  - `InheritedWidget`
  - `RenderWidget`
- Impls:
  - `WidgetState`
  - `RenderState`
  - `ParentData`
- Preserving state:
  - `LocalKey`
  - Position in children list
- Basic event detectors:
  - `KeyboardEventDetector`
  - `PointerListener`
  - `PointerRegion`
- Basic widgets:
  - `Text`
  - `Center`
  - `Row`
  - `Column`
  - `Flex`
  - `Stack`
  - `SizedBox`
  - `LimitedBox`
  - `ConstrainedBox`


For more features see `examples`.


## 🦀 Counter - Example

Obligatory crab counter! From `examples/crab_counter.rs`.

```rust

#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;
use misc::Button;

#[derive(ViewWidget)]
struct CrabCounter;

impl WidgetState for CrabCounter {
    type State = isize;

    fn create_state(&self) -> Self::State { 0 }
}

impl ViewWidget for CrabCounter {
    fn build<'w>(&'w self, cx: BuildCx<'w, Self>) -> Self::Widget<'w> {
        Column::builder()
            .space_between(60.0)
            .main_axis_size(MainAxisSize::Max)
            .cross_axis_size(CrossAxisSize::Max)
            .main_axis_alignment(MainAxisAlignment::Center)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .children((
                Text::new(format!("{} 🦀", *cx.state()))
                    .size(100.0)
                    .weight(FontWeight::BOLD),
                Row::builder()
                    .space_between(10.0)
                    .children((
                        Button {
                            label: Text::new("+").size(30.),
                            on_click: || *cx.state_mut() += 1,
                        },
                        Button {
                            label: Text::new("-").size(30.),
                            on_click: || *cx.state_mut() -= 1,
                        },
                    )),
            ))
    }
}

fn main() {
    run_app(CrabCounter);
}
```

<p align="center"><img src="assets/crab_counter.png" height="400px" alt="screenshot of application running above code"/></p>

*<p align="center">Crab counter running on MacOS</p>*

## Credits


Frui was inspired by Flutter's widget architecture and API, and builds upon the work done in Druid, which powers much of the back-end and has influenced the implementation of many widgets. The contributions of both Flutter and Druid were essential to the development of Frui, and for those – thank you!

## License

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
