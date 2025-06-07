use std::str::FromStr;

use dioxus::{logger::tracing, prelude::*};
use tiling_mino_solver::{Board, Mino, Shape};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    tracing::info!("Starting the application");
    let mut minos = use_signal(Vec::new);
    let mut board = use_signal(|| Board::new(Shape::new(vec![vec![false; 9]; 9])));
    let handle_mino_maker = move |new_shape: Shape| {
        tracing::info!("Button pushed with event: {:?}", new_shape);
        let new_mino = Mino::new('a', new_shape.clone());
        minos.write().push(new_mino);
    };
    rsx! {
        MinoMaker { minos: minos(), handle_push_button: handle_mino_maker }
        BoardMaker {
            board: board(),
            handle_click: move |(x, y)| {
                tracing::info!("Clicked on board cell ({}, {})", x, y);
                board.write().shape.toggle(x, y);
            },
        }
    }
}

#[component]
fn BoardMaker(board: Board, handle_click: EventHandler<(usize, usize)>) -> Element {
    tracing::info!("Rendering BoardMaker component");
    rsx! {
        div { "Board Maker" }
        Lattice { shape: board.shape, cell_pixel: 20, handle_click }
    }
}

#[component]
fn MinoMaker(minos: Vec<Mino>, handle_push_button: EventHandler<Shape>) -> Element {
    tracing::info!("Rendering MinoMaker component");
    println!("This is println macro");
    let mut new_shape = use_signal(|| Shape::from_str(".#.\n.##\n").unwrap());
    let handle_click_with_toggle = move |(x, y)| {
        tracing::info!("Clicked on cell ({}, {})", x, y);
        new_shape.write().toggle(x, y);
    };
    rsx! {
        Lattice {
            shape: new_shape(),
            cell_pixel: 100,
            handle_click: handle_click_with_toggle,
        }
        button { onclick: move |_| handle_push_button.call(new_shape()), "Make Mino" }
        for mino in minos {
            div { "new" }
            Lattice { shape: mino.shape, cell_pixel: 50, handle_click: |_| {} }
        }
    }
}

#[component]
fn Lattice(shape: Shape, cell_pixel: usize, handle_click: EventHandler<(usize, usize)>) -> Element {
    tracing::debug!("Rendering mutable lattice with shape: {:?}", shape);
    let style = format!(
        "display: grid; grid-template-columns: repeat({}, {}px); grid-template-rows: repeat({}, {}px);",
        shape.width(),
        cell_pixel,
        shape.height(),
        cell_pixel
    );
    rsx! {
        div { class: "lattice", style: style.clone(),
            for (x , y , is_wall) in shape.coordinates() {
                LatticeCell {
                    is_wall,
                    handle_click: move |_| {
                        handle_click.call((x, y));
                    },
                }
            }
        }
    }
}

#[component]
fn LatticeCell(is_wall: bool, handle_click: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div {
            class: "lattice-cell",
            border: "1px solid #000",
            padding: "10px",
            background_color: if is_wall { "gray" } else { "white" },
            onclick: move |evt| handle_click.call(evt),
        }
    }
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
