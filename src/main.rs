use std::{cell::Cell, str::FromStr};

use dioxus::{
    html::mo::{self, movablelimits},
    prelude::*,
};
use hot_dog::Shape;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let shape = Shape::from_str(".#...\n.###.\n").unwrap();
    rsx! {
        Lattice { shape, cell_pixel: 100 }
        Counter {}
    }
}

#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        div { "Counter: {count}" }
        button { onclick: move |_| count += 1, "Increase" }
    }
}

#[component]
fn Lattice(shape: Shape, cell_pixel: usize) -> Element {
    let shape = use_signal(|| shape.clone());
    let style = format!(
        "display: grid; grid-template-columns: repeat({}, {}px); grid-template-rows: repeat({}, {}px);",
        shape().width(),
        cell_pixel,
        shape().height(),
        cell_pixel
    );
    rsx! {
        div { class: "lattice", style,
            for (x , y , is_wall) in shape().coordinates() {
                LatticeCell {
                    onclick: move |_| shape().put_on(x, y, !is_wall),
                    is_wall,
                }
            }
        }
        FancyButton { onclick: move |event| println!("{event:?}"), other: true }
    }
}

fn LatticeCell(props: CellProps) -> Element {
    rsx! {
        div {
            class: "lattice-cell",
            border: "1px solid #000",
            padding: "10px",
            background_color: if props.is_wall { "gray" } else { "white" },
        }
    }
}

#[derive(PartialEq, Clone, Props)]
struct CellProps {
    onclick: EventHandler<MouseEvent>,
    is_wall: bool,
}

#[derive(PartialEq, Clone, Props)]
struct FancyButtonProps {
    onclick: EventHandler<MouseEvent>,
    other: bool,
}

fn FancyButton(props: FancyButtonProps) -> Element {
    rsx! {
        button {
            class: "fancy-button",
            onclick: move |evt| props.onclick.call(evt),
            "click me pls."
        }
    }
}
