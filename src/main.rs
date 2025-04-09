use leptos::{
    ev::{self, MouseEvent}, logging::log, prelude::*, svg::Svg
};

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    const SVG_SIZE: i32 = 500;
    let viewbox = move || format!("0 0 {SVG_SIZE} {SVG_SIZE}");

    let (mousedown, set_mousedown) = signal(false);
    let (mousepos, set_mousepos) = signal((0.0, 0.0));
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
        set_mousepos.set((x, y));
    };

    view! {
        <div style="display: flex; justify-content: center; align-items: center;">
            <svg node_ref=svg viewBox=viewbox style="width: 50%; height: 50%;" on:mousemove=mousemove>
                <Curve mousedown=mousedown mousepos=mousepos
                    a1x=250.0 a1y=50.0
                    c1x=210.0 c1y=100.0
                    c2x=300.0 c2y=200.0
                    a2x=300.0 a2y=400.0
                />
                <Curve mousedown=mousedown mousepos=mousepos
                    a1x=50.0 a1y=250.0
                    c1x=410.0 c1y=200.0
                    c2x=100.0 c2y=200.0
                    a2x=400.0 a2y=300.0
                />
            </svg>
        </div>
    }
}

#[component]
fn Curve(
    mousedown: ReadSignal<bool>,
    mousepos: ReadSignal<(f64, f64)>,
    a1x: f64,
    a1y: f64,
    c1x: f64,
    c1y: f64,
    c2x: f64,
    c2y: f64,
    a2x: f64,
    a2y: f64,
) -> impl IntoView {
    let (a1, set_a1) = signal((a1x, a1y));
    let (c1, set_c1) = signal((c1x, c1y));
    let (c2, set_c2) = signal((c2x, c2y));
    let (a2, set_a2) = signal((a2x, a2y));

    Effect::new(move |_| {
        log!("a1: {:#?}, c1: {:#?}, c2: {:#?}, a2: {:#?}", a1.get(), c1.get(), c2.get(), a2.get());
    });

    let path = move || {
        let a1 = a1.get();
        let c1 = sub_f64_tup(c1.get(), a1);
        let c2 = sub_f64_tup(c2.get(), a1);
        let a2 = sub_f64_tup(a2.get(), a1);
        format!("M {} {} c {} {} {} {} {} {}", a1.0, a1.1, c1.0, c1.1, c2.0, c2.1, a2.0, a2.1)
    };

    view! {
        <path d=path stroke="black" stroke-width="2" fill="none" />
        <ControlPoint stroke="blue" pos=a1 set_pos=set_a1 mousedown=mousedown mousepos=mousepos />
        <ControlPoint stroke="blue" pos=a2 set_pos=set_a2 mousedown=mousedown mousepos=mousepos />
        <ControlPoint stroke="red" pos=c1 set_pos=set_c1 mousedown=mousedown mousepos=mousepos />
        <ControlPoint stroke="red" pos=c2 set_pos=set_c2 mousedown=mousedown mousepos=mousepos />
    }
}

fn sub_f64_tup(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 - b.0, a.1 - b.1)
}

#[component]
fn ControlPoint<'a>(
    pos: ReadSignal<(f64, f64)>,
    set_pos: WriteSignal<(f64, f64)>,
    mousedown: ReadSignal<bool>,
    mousepos: ReadSignal<(f64, f64)>,
    stroke: &'a str,
) -> impl IntoView {
    const SIZE: i32 = 5;

    let (dragging, set_dragging) = signal(false);
    let x = move || pos.get().0 - (SIZE as f64 / 2.0);
    let y = move || pos.get().1 - (SIZE as f64 / 2.0);

    Effect::new(move |_| {
        if !mousedown.get() {
            set_dragging.set(false);
        }
    });

    Effect::new(move |_| {
        if dragging.get() {
            set_pos.set(mousepos.get());
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
