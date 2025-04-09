use bezier::CubicBezier;
use leptos::{component, mount::mount_to_body, prelude::signal, view, IntoView};
use point::Point;

pub mod point;
pub mod bezier;
pub mod svg;

use svg::SvgDisplay;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (b1, set_b1) = signal(CubicBezier::new(
        Point::new(50., 250.),
        Point::new(100., 250.),
        Point::new(400., 250.),
        Point::new(450., 250.),
    ));

    let (b2, set_b2) = signal(CubicBezier::new(
        Point::new(110., 195.),
        Point::new(205., 495.),
        Point::new(290., 20.),
        Point::new(395., 300.),
    ));

    view! {
        <SvgDisplay b1=b1 b2=b2 set_b1=set_b1 set_b2=set_b2 />
    }
}

