//! This example shows how to use [`Transform`] widget.
//!
//! Do note that this widget is not yet implemented properly and for now it
//! exists just to showcase that widgets are being hit tested correctly.

#![feature(type_alias_impl_trait)]

use frui::prelude::*;
use frui::render::*;

mod counter;

use counter::Counter;

#[derive(ViewWidget)]
struct CounterRotated;

impl ViewWidget for CounterRotated {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Self::Widget<'w> {
        let screen_width = 500.;
        let screen_height = 400.;

        // Transform widget is not yet fully implemented, but I decided to
        // inlcude it to showcase that widget hit testing is correctly
        // implemented!
        Transform(
            Affine::translate((screen_width / 2., screen_height / 2.))
                * Affine::rotate(std::f64::consts::FRAC_PI_8)
                * Affine::translate((-screen_width / 2., -screen_height / 2.)),
            Counter,
        )
    }
}

fn main() {
    run_app(CounterRotated)
}
