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
    let push_mino = move |new_shape: Shape| {
        tracing::info!("Button pushed with event: {:?}", new_shape);
        let new_mino = Mino::new('a', new_shape.clone());
        minos.write().push(new_mino);
    };
    rsx! {
        MinoMaker { minos: minos(), handle_push_button: push_mino }
        BoardMaker {
            board: board(),
            handle_click: move |(x, y)| {
                tracing::info!("Clicked on board cell ({}, {})", x, y);
                board.write().shape.toggle(x, y);
            },
        }
        button {
            onclick: move |_| {
                tracing::info!("Solve button clicked");
                let solved: Option<Board> = board().tile_parallel(&minos());
                if let Some(s) = solved {
                    *board.write() = s;
                } else {
                    tracing::info!("No solution found");
                }
            },
            "Solve"
        }
    }
}

#[component]
fn BoardMaker(board: Board, handle_click: EventHandler<(usize, usize)>) -> Element {
    tracing::info!("Rendering BoardMaker component");
    rsx! {
        div { "Board Maker" }
        Lattice { color_shape: board.into(), cell_pixel: 20, handle_click }
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
            color_shape: new_shape().into(),
            cell_pixel: 100,
            handle_click: handle_click_with_toggle,
        }
        button { onclick: move |_| handle_push_button.call(new_shape()), "Make Mino" }
        for mino in minos {
            div { "new" }
            Lattice {
                color_shape: mino.shape.into(),
                cell_pixel: 50,
                handle_click: |_| {},
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Blue = 0x0000FF,
    Cyan = 0x00FFFF,
    Green = 0x00FF00,
    Gray = 0x808080,
    Purple = 0x800080,
    Red = 0xFF0000,
    Yellow = 0xFFFF00,
    White = 0xFFFFFF,
    Black = 0x000000,
}

impl Color {
    fn to_string(&self) -> String {
        format!("#{:06X}", *self as usize)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ColorShape(Vec<Vec<Color>>);

impl Into<ColorShape> for Shape {
    fn into(self) -> ColorShape {
        let mut color_shape = vec![vec![Color::White; self.width()]; self.height()];
        for (x, y, is_wall) in self.coordinates() {
            if is_wall {
                color_shape[y][x] = Color::Gray;
            }
        }
        ColorShape(color_shape)
    }
}

impl Into<ColorShape> for Board {
    fn into(self) -> ColorShape {
        self.shape.into()
    }
}

impl ColorShape {
    fn coordinates(&self) -> impl Iterator<Item = (usize, usize, Color)> + '_ {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &color)| (x, y, color)))
    }

    fn width(&self) -> usize {
        self.0.first().map_or(0, |row| row.len())
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

#[component]
fn Lattice(
    color_shape: ColorShape,
    cell_pixel: usize,
    handle_click: EventHandler<(usize, usize)>,
) -> Element {
    tracing::trace!("Rendering mutable lattice with shape: {:?}", color_shape);
    let style = format!(
        "display: grid; grid-template-columns: repeat({}, {}px); grid-template-rows: repeat({}, {}px);",
        color_shape.width(),
        cell_pixel,
        color_shape.height(),
        cell_pixel
    );
    rsx! {
        div { class: "lattice", style: style.clone(),
            for (x , y , color) in color_shape.coordinates() {
                LatticeCell {
                    handle_click: move |_| {
                        handle_click.call((x, y));
                    },
                    color: color.to_string(),
                }
            }
        }
    }
}

#[component]
fn LatticeCell(color: String, handle_click: EventHandler<MouseEvent>) -> Element {
    tracing::trace!("Color enum values: {:?}", color);
    rsx! {
        div {
            class: "lattice-cell",
            border: "1px solid #000",
            padding: "10px",
            background_color: color,
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
