<h1 align="center"><img src="assets/logo.svg" height="80px" alt="Frui"/></h1>

*<p align="center">Reading: "Fr" as in "fruit" and "ui" as in "you" and "I".</p>*

<p align="center">
<a href="https://crates.io/crates/frui"><img src="https://img.shields.io/crates/v/frui.svg" alt="Latest version"/></a>
<a href="https://github.com/emilk/egui/blob/master/LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT"/></a>
<a href="https://github.com/emilk/egui/blob/master/LICENSE-APACHE"><img src="https://img.shields.io/badge/license-Apache-blue.svg" alt="Apache"/></a>
</p>

## What is Frui?

Frui is a developer-friendly UI framework that makes building user interfaces easy and productive. It's based on Flutter architecture and is written in Rust!

## Example

```rust
#![feature(min_specialization)]
#![feature(type_alias_impl_trait)]

use frui::prelude::*;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        Center::child(Text::new("Hello, World!"))
    }
}

fn main() {
    run_app(App);
}
```


## Features

Frui, as of now, is a PoC, and has only implemented the most essential parts of the API. There is a lot of work that needs to be done, especially in terms of back-end considerations, optimizations, and widget implementations (widgets like buttons, gesture detectors, theming, flex wrappers,  etc.).

- [x] `ViewWidget` (based on [`StatelessWidget`](https://api.flutter.dev/flutter/widgets/StatelessWidget-class.html) and [`StatefulWidget`](https://api.flutter.dev/flutter/widgets/StatefulWidget-class.html))
- [x] `InheritedWidget` (based on [`InheritedWidget`](https://api.flutter.dev/flutter/widgets/InheritedWidget-class.html))
- [x] `LocalKey` (based on [`Key`](https://api.flutter.dev/flutter/foundation/Key-class.html))
- [x] Scheduling state updates
- [x] Basic event detection (`KeyboardEventDetector` / mouse events through `event` method)
- [x] Basic layout widgets (`Column`, `Row`, `Center`)
- [ ] Documentation and tutorials
- [ ] Event passing, handling focus, Z-layers
- [ ] Optimizations: widget-rebuild order based on depth, layout & painting
- [ ] Library of widgets common to all visual langauges provided out of the box (`Column`, `Row`, `GestureDetector`, `Scroll`, etc.)
- [ ] Separate widget library implementing one of the design languages (e.g. Material Design)


## 🦀 Counter - Example

Obligatory crab counter! From `examples/crab_counter.rs`.

```rust
#![feature(min_specialization)]
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
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w> {
        Column::builder()
            .space_between(60.0)
            .main_axis_size(MainAxisSize::Max)
            .cross_axis_size(CrossAxisSize::Max)
            .main_axis_alignment(MainAxisAlignment::Center)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .children((
                Text::new(format!("{} 🦀", *ctx.state()))
                    .size(100.0)
                    .weight(FontWeight::BOLD),
                Row::builder()
                    .space_between(10.0)
                    .children((
                        Button {
                            label: Text::new("+").size(30.),
                            on_click: || *ctx.state_mut() += 1,
                        },
                        Button {
                            label: Text::new("-").size(30.),
                            on_click: || *ctx.state_mut() -= 1,
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

*<p align="center">Crabs counter running on MacOS</p>*

## Credits

Frui wouldn't exist without Flutter and its widget architecture, which inspired Frui's API.

Frui also wouldn't exist without prior work done on Druid - which powers most of the back-end. Many widgets share some of the implementation details with it as well.

Thank you!


## License

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))