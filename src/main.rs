mod board;

use board::{Board, Coords};

use leptos::prelude::*;
use std::f64;
use wasm_bindgen::prelude::*;

const CELL_SIZE: usize = 100;

fn main() {
    console_error_panic_hook::set_once();

    let b: Board = "
        -----
        --x--
        --x--
        --x--
        -----
    "
    .try_into()
    .unwrap();
    let b = b.next();

    let canvas_height = CELL_SIZE * b.dim_y();
    let canvas_width = CELL_SIZE * b.dim_x();

    mount_to_body(move || view! { <App canvas_height=canvas_height canvas_width=canvas_width/> });

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    draw(context, b);
}

#[component]
fn App(canvas_height: usize, canvas_width: usize) -> impl IntoView {
    view! {
        <canvas id="canvas" height=canvas_height width=canvas_width style="background-color: black;"></canvas>
    }
}

fn draw(context: web_sys::CanvasRenderingContext2d, b: Board) {
    context.set_fill_style_str("green");

    for y in 0..b.dim_y() {
        for x in 0..b.dim_x() {
            let alive = b.alive(&Coords { x, y });
            if alive {
                let canvas_y = y * CELL_SIZE;
                let canvas_x = x * CELL_SIZE;
                context.fill_rect(
                    canvas_x as f64,
                    canvas_y as f64,
                    CELL_SIZE as f64,
                    CELL_SIZE as f64,
                );
            }
        }
    }
}
