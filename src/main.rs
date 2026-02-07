use std::str::FromStr;

use dioxus::{logger::tracing, prelude::*};
use tiling_mino_solver::{Board, Mino, Shape};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    tracing::info!("Starting the application");
    rsx! {
        // The Stylesheet component inserts a style link into the head of the document
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/tailwind.css"),
        }
        div {
            class: "p-5 bg-gray-600",
            FlexBoxTest {  }
            TailingMinoSolver { }
        }
    }
}

#[component]
fn FlexBoxTest() -> Element {
    rsx! {
        div { class: "p-10 flex",
            div { class: "bg-sky-600 basis-1/3", "1:ベイシス3分の1ィ" }
            div { class: "basis-1/3", "2:ベイシス3分の1ィ" }
            div { class: "basis-2/3", "3:ベイシス3分の2ィ" }
            div { class: "basis-1/3", "4:ベイシス3分の1ィ" }
        }
    }
}

#[component]
fn TailingMinoSolver() -> Element {
    let mut minos = use_signal(Vec::new);
    let mut board = use_signal(|| Board::new(Shape::new(vec![vec![false; 9]; 9])));
    let mut solved_board = use_signal(|| Board::new(Shape::new(vec![vec![false; 9]; 9])));
    let make_mino = move |mut new_shape: Shape| {
        let current_len = minos().len();
        tracing::info!("Current number of minos: {}", current_len);
        tracing::info!("Button pushed with event: {:?}", new_shape);
        new_shape.trim();
        let new_mino = Mino::new(CHAR_PALETTE[current_len], new_shape);
        minos.write().push(new_mino);
    };
    let toggle_board_cell = move |(x, y)| {
        tracing::info!("Clicked on board cell ({}, {})", x, y);
        board.write().shape.toggle(x, y);
    };
    let solve_tiling = move |_| {
        tracing::info!("Solve button clicked");
        let answer: Option<Board> = board().tile_parallel(&minos());
        if let Some(a) = answer {
            *solved_board.write() = a;
        } else {
            tracing::info!("No solution found");
        }
    };
    rsx! {
        MinoMaker { minos: minos(), handle_push_button: make_mino }
        div { class: "flex flex-row",
            div { class: "basis-1/2",
                div { "Board Maker" }
                BoardMaker { board: board(), handle_click: toggle_board_cell }
                button {
                    onclick: solve_tiling,
                    "Solve"
                }
            }
            div { class: "basis-1/2",
                div { "Answer" }
                BoardMaker { board: solved_board(), handle_click: |_| {} }
            }
        }
    }
}

#[component]
fn PlusButton(handle_click: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "rounded-full size-6 bg-blue-500 hover:bg-blue-700 text-white font-bold flex items-center justify-center",
            onclick: move |evt| handle_click.call(evt),
            svg {
                class: "w-4 h-4",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "3",
                stroke_linecap: "round",
                line {
                    x1: "12",
                    y1: "5",
                    x2: "12",
                    y2: "19",
                }
                line {
                    x1: "5",
                    y1: "12",
                    x2: "19",
                    y2: "12",
                }
            }
        }
    }
}

#[component]
fn BoardMaker(board: Board, handle_click: EventHandler<(usize, usize)>) -> Element {
    tracing::info!("Rendering BoardMaker component");
    rsx! {
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
        div { class : "flex flex-row flex-wrap",
            for mino in minos {
                Lattice {
                    color_shape: mino.into(),
                    cell_pixel: 50,
                    handle_click: |_| {},
                }
            }
            PlusButton { handle_click: move |_| handle_push_button.call(new_shape()) }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Blue = 0x0000FF,
    Cyan = 0x00FFFF,
    Green = 0x00FF00,
    Purple = 0x800080,
    Red = 0xFF0000,
    Yellow = 0xFFFF00,

    White = 0xFFFFFF,
    Black = 0x000000,
    Gray = 0x808080,
}

const CHAR_PALETTE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

impl Color {
    fn tailwind_bg(&self) -> &'static str {
        match self {
            Color::Blue => "bg-blue-500",
            Color::Cyan => "bg-cyan-500",
            Color::Green => "bg-green-500",
            Color::Purple => "bg-purple-500",
            Color::Red => "bg-red-500",
            Color::Yellow => "bg-yellow-500",
            Color::White => "bg-white",
            Color::Black => "bg-black",
            Color::Gray => "bg-gray-500",
        }
    }
    fn style_color(&self) -> String {
        format!("#{:06X}", *self as usize)
    }

    fn from_char(c: char) -> Self {
        match c {
            'a' => Color::Blue,
            'b' => Color::Cyan,
            'c' => Color::Green,
            'd' => Color::Red,
            'e' => Color::Purple,
            'f' => Color::Yellow,
            _ => Color::Gray, // Default to gray for unknown characters
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ColorShape(Vec<Vec<Color>>);

impl From<Shape> for ColorShape {
    fn from(val: Shape) -> Self {
        let mut color_shape = vec![vec![Color::White; val.width()]; val.height()];
        for (x, y, is_wall) in val.coordinates() {
            if is_wall {
                color_shape[y][x] = Color::Gray;
            }
        }
        ColorShape(color_shape)
    }
}

impl From<Board> for ColorShape {
    fn from(board: Board) -> Self {
        let colors = board
            .cell_state_matrix()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell| match cell {
                        tiling_mino_solver::CellState::Occupied(c) => Color::from_char(c),
                        tiling_mino_solver::CellState::Wall => Color::Gray,
                        tiling_mino_solver::CellState::Vacant => Color::White,
                    })
                    .collect()
            })
            .collect();
        ColorShape(colors)
    }
}

impl From<Mino> for ColorShape {
    fn from(mino: Mino) -> Self {
        let mut color_shape = vec![vec![Color::White; mino.shape.width()]; mino.shape.height()];
        for (x, y, is_wall) in mino.shape.coordinates() {
            if is_wall {
                tracing::info!("Setting color for cell ({}, {}) to {}", x, y, mino.name);
                color_shape[y][x] = Color::from_char(mino.name);
            }
        }
        ColorShape(color_shape)
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
        "display: grid; grid-template-columns:repeat({}, {}px); grid-template-rows:repeat({}, {}px);",
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
                    color: color.tailwind_bg(),
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
            class: "{color} border",
            onclick: move |evt| handle_click.call(evt),
        }
    }
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;
    #[test]
    fn test() {
        let w = "world";
        assert_rsx_eq(
            rsx! {
                div { "Hello {w}" }
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
