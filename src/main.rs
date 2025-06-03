mod board;
use anyhow::Result;
use board::{Board, Coords};
use leptos::{html::Canvas, prelude::*};
use std::f64;
use wasm_bindgen::prelude::*;

const CELL_SIZE: usize = 10;

fn main() -> Result<()> {
    console_error_panic_hook::set_once();

    let board = Board::new(Coords { x: 120, y: 120 }, None)?;
    let add_board = Board::gosper();
    let initial_board = board.add(add_board, Coords { x: 15, y: 15 })?;

    let canvas_height = CELL_SIZE * initial_board.dim_y();
    let canvas_width = CELL_SIZE * initial_board.dim_x();

    mount_to_body(move || {
        view! {
            <App
                canvas_height=canvas_height
                canvas_width=canvas_width
                initial_board=initial_board
            />
        }
    });

    Ok(())
}

#[component]
fn App(canvas_height: usize, canvas_width: usize, initial_board: Board) -> impl IntoView {
    // Create reactive state for the board
    let (board, set_board) = signal(initial_board);
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    // State to control auto-play
    let (is_running, set_is_running) = signal(false);
    let (interval_id, set_interval_id) = signal(None::<i32>);
    let (interval_seconds, set_interval_seconds) = signal(0.05f64);

    // Effect to redraw canvas whenever board changes
    Effect::new(move |_| {
        // Get canvas and context
        let canvas = canvas_ref.get().unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        draw(context, board.get());
    });

    // Manual step function
    let step = move |_: web_sys::MouseEvent| {
        set_board.update(|b| *b = b.next());
    };

    // Function to start the interval with current settings
    let start_interval = {
        let set_board = set_board.clone();
        let set_interval_id = set_interval_id.clone();
        move || {
            let callback = Closure::wrap(Box::new({
                let set_board = set_board.clone();
                move || {
                    set_board.update(|b| *b = b.next());
                }
            }) as Box<dyn FnMut()>);

            let id = web_sys::window()
                .unwrap()
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    (interval_seconds.get() * 1000.0) as i32,
                )
                .unwrap();

            callback.forget();
            set_interval_id.set(Some(id));
        }
    };

    // Function to stop the interval
    let stop_interval = move || {
        if let Some(id) = interval_id.get() {
            web_sys::window().unwrap().clear_interval_with_handle(id);
        }
        set_interval_id.set(None);
    };

    // Auto-play toggle function
    let toggle_auto_play = {
        let start_interval = start_interval.clone();
        move |_: web_sys::MouseEvent| {
            if is_running.get() {
                stop_interval();
                set_is_running.set(false);
            } else {
                start_interval();
                set_is_running.set(true);
            }
        }
    };

    // Interval control functions
    let decrease_interval = {
        let start_interval = start_interval.clone();
        let stop_interval = stop_interval.clone();
        move |_: web_sys::MouseEvent| {
            set_interval_seconds.update(|seconds| {
                *seconds = (*seconds - 0.01).max(0.0);
            });
            // Restart interval if running
            if is_running.get() {
                stop_interval();
                start_interval();
            }
        }
    };

    let increase_interval = {
        let start_interval = start_interval.clone();
        let stop_interval = stop_interval.clone();
        move |_: web_sys::MouseEvent| {
            set_interval_seconds.update(|seconds| {
                *seconds += 0.01;
            });
            // Restart interval if running
            if is_running.get() {
                stop_interval();
                start_interval();
            }
        }
    };

    let on_interval_input = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
        if let Ok(value) = input.value().parse::<f64>() {
            set_interval_seconds.set(value.max(0.0));
            // Restart interval if running
            if is_running.get() {
                stop_interval();
                start_interval();
            }
        }
    };

    view! {
        <div>
            <canvas
                id="canvas"
                node_ref=canvas_ref
                height=canvas_height
                width=canvas_width
                style="background-color: #333333; display: block;"
            ></canvas>
            <div style="margin-top: 10px;">
                <button on:click=step style="padding: 10px 20px; font-size: 16px; margin-right: 10px;">
                    "Next"
                </button>
                <button on:click=toggle_auto_play style="padding: 10px 20px; font-size: 16px;">
                    {move || if is_running.get() { "Stop" } else { "Start" }}
                </button>
            </div>
            <div style="margin-top: 10px; display: flex; align-items: center; gap: 10px;">
                <span>"Interval (s):"</span>
                <button on:click=decrease_interval style="padding: 5px 10px; font-size: 14px;">"-"</button>
                <input
                    type="number"
                    step="0.1"
                    min="0"
                    prop:value=move || format!("{:.2}", interval_seconds.get())
                    on:input=on_interval_input
                    style="width: 80px; padding: 5px; text-align: center;"
                />
                <button on:click=increase_interval style="padding: 5px 10px; font-size: 14px;">"+"</button>
            </div>
        </div>
    }
}

fn draw(context: web_sys::CanvasRenderingContext2d, b: Board) {
    // Draw alive cells
    context.set_fill_style_str("green");
    for y in 0..b.dim_y() {
        for x in 0..b.dim_x() {
            let alive = b.alive(&Coords { x, y });
            if alive {
                context.set_fill_style_str("green");
            } else {
                context.set_fill_style_str("black");
            }
            context.fill_rect(
                (x * CELL_SIZE) as f64,
                (y * CELL_SIZE) as f64,
                (CELL_SIZE - 1) as f64,
                (CELL_SIZE - 1) as f64,
            );
        }
    }
}
