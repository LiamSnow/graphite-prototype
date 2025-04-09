use leptos::{
    ev::{self, MouseEvent},
    prelude::*,
    svg::Svg,
};

use crate::{bezier::CubicBezier, point::Point};

#[component]
pub fn SvgDisplay(
    b1: ReadSignal<CubicBezier>,
    set_b1: WriteSignal<CubicBezier>,
    b2: ReadSignal<CubicBezier>,
    set_b2: WriteSignal<CubicBezier>
) -> impl IntoView {
    const SVG_SIZE: i32 = 500;
    let viewbox = move || format!("0 0 {SVG_SIZE} {SVG_SIZE}");

    let (mousedown, set_mousedown) = signal(false);
    let (mousepos, set_mousepos) = signal(Point::default());
    let svg = NodeRef::<Svg>::new();

    window_event_listener(ev::mousedown, move |_| {
        set_mousedown.set(true);
    });

    window_event_listener(ev::mouseup, move |_| {
        set_mousedown.set(false);
    });

    let mousemove = move |ev: MouseEvent| {
        let svg = svg.try_get().unwrap().unwrap();
        let x = ev.offset_x() as f64 / svg.client_width() as f64 * 500.0; //FIXME make consts
        let y = ev.offset_y() as f64 / svg.client_height() as f64 * 500.0;
        set_mousepos.set(Point::new(x, y));
    };

    view! {
        <div style="display: flex; justify-content: center; align-items: center;">
            <svg node_ref=svg viewBox=viewbox style="width: 50%; height: 50%;" on:mousemove=mousemove>
                <Curve stroke1="cyan" stroke2="#F535AA" b=b1 set_b=set_b1 mousedown=mousedown mousepos=mousepos />
                <Curve stroke1="red" stroke2="blue" b=b2 set_b=set_b2 mousedown=mousedown mousepos=mousepos />
            </svg>
        </div>
    }
}

#[component]
fn Curve<'a>(
    mousedown: ReadSignal<bool>,
    mousepos: ReadSignal<Point>,
    b: ReadSignal<CubicBezier>,
    set_b: WriteSignal<CubicBezier>,
    stroke1: &'a str,
    stroke2: &'a str,
) -> impl IntoView {
    let paths = move || {
        let b = b.read();
        let (l, r) = b.split();
        (l.to_path(), r.to_path())
    };

    let path1 = move || paths().0;
    let path2 = move || paths().1;

    view! {
        <path d=path1 stroke=stroke1 stroke-width="2" fill="none" />
        <path d=path2 stroke=stroke2 stroke-width="2" fill="none" />
        <ControlPoint i=0 stroke="#2C9DBC" b=b set_b=set_b mousedown=mousedown mousepos=mousepos />
        <ControlPoint i=1 stroke="red"     b=b set_b=set_b mousedown=mousedown mousepos=mousepos />
        <ControlPoint i=2 stroke="red"     b=b set_b=set_b mousedown=mousedown mousepos=mousepos />
        <ControlPoint i=3 stroke="#2C9DBC" b=b set_b=set_b mousedown=mousedown mousepos=mousepos />
    }
}

#[component]
fn ControlPoint<'a>(
    b: ReadSignal<CubicBezier>,
    set_b: WriteSignal<CubicBezier>,
    i: usize,
    mousedown: ReadSignal<bool>,
    mousepos: ReadSignal<Point>,
    stroke: &'a str,
) -> impl IntoView {
    const SIZE: i32 = 5;

    let (dragging, set_dragging) = signal(false);
    let x = move || {
        let b = b.read();
        b.get_index(i).x - (SIZE as f64 / 2.0)
    };
    let y = move || {
        let b = b.read();
        b.get_index(i).y - (SIZE as f64 / 2.0)
    };

    Effect::new(move |_| {
        if !mousedown.get() {
            set_dragging.set(false);
        }
    });

    Effect::new(move |_| {
        if dragging.get() {
            let mut b = set_b.write();
            b.set_index(i, mousepos.get());
        }
    });

    view! {
        <rect width=SIZE height=SIZE x=x y=y
            stroke-width="1" stroke=stroke fill="white"
            on:mousedown=move |_| set_dragging.set(true)
            style="cursor: pointer"
        />
    }
}
