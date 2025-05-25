use std::str::FromStr;

use dioxus::{logger::tracing, prelude::*};
use hot_dog::Shape;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let shape = Shape::from_str(".#...\n.###.\n").unwrap();
    tracing::info!("Starting the application");
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
    let mut shape = use_signal(|| shape.clone());
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
                    onclick: move |_| {
                        tracing::info!("Clicked on cell ({}, {}, {})", x, y, is_wall);
                        shape.write().toggle(x, y);
                    },
                    is_wall,
                }
            }
        }
    }
}

fn LatticeCell(props: CellProps) -> Element {
    tracing::debug!("Rendering cell with is_wall: {}", props.is_wall);
    rsx! {
        div {
            class: "lattice-cell",
            border: "1px solid #000",
            padding: "10px",
            background_color: if props.is_wall { "gray" } else { "white" },
            onclick: move |evt| props.onclick.call(evt),
        }
    }
}

#[derive(PartialEq, Clone, Props)]
struct CellProps {
    onclick: EventHandler<MouseEvent>,
    is_wall: bool,
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;
    #[test]
    fn test() {
        assert_rsx_eq(
            rsx! {
                div { "Hello world" }
                div { "Hello world" }
            },
            rsx! {
                for _ in 0..2 {
                    div { "Hello world" }
                }
            },
        )
    }
    fn assert_rsx_eq(first: Element, second: Element) {
        let first = dioxus_ssr::render_element(first);
        let second = dioxus_ssr::render_element(second);
        pretty_assertions::assert_eq!(first, second);
    }
}
