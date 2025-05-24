use std::str::FromStr;

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
            for (_ , _ , is_wall) in shape().coordinates() {
                LatticeCell { is_wall }
            }
        }
    }
}

#[component]
fn LatticeCell(is_wall: bool) -> Element {
    rsx! {
        div {
            class: "lattice-cell",
            border: "1px solid #000",
            padding: "10px",
            background_color: if is_wall { "gray" } else { "white" },
        }
    }
}
