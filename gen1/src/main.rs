use nannou::geom::rect::Rect;
use nannou::prelude::*;
use nannou::rand::prelude::*;
use nannou::rand::thread_rng;

// All the good names for things are taken
#[derive(Debug)]
struct ThisIsMyBox {
    x: i32,
    y: i32,
    size: i32,
}

struct Model {
    gw: i32,
    gh: i32,
    boxes: Vec<ThisIsMyBox>,
}

fn main() {
    nannou::app(model)
        .size(1200, 1200)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    // Grid dimensions
    const GW: i32 = 400;
    const GH: i32 = 400;
    const SIZE: usize = (GW * GH) as usize;

    // Grid occupancy
    let mut array: [bool; SIZE] = [false; SIZE];
    let boxes = fill_grid(&mut array, GH, GW);

    Model {
        boxes,
        gw: GW,
        gh: GH,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let Model {
        ref gw,
        ref gh,
        ref boxes,
    } = *model;

    let boundary = app.window_rect();
    let w_mag = boundary.w() / (*gw as f32);
    let h_mag = boundary.h() / (*gh as f32);

    let draw = app.draw();
    draw.background().color(PLUM);

    for b in boxes.iter() {
        let r =
            Rect::from_w_h((b.size as f32) * w_mag, (b.size as f32) * h_mag).top_left_of(boundary);
        draw.rect()
            .xy(r.xy() + pt2((b.x as f32) * w_mag, (b.y as f32) * h_mag * -1.0))
            .wh(r.wh())
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(1.0);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn make_offset(size: i32) -> i32 {
    ((size as f32) / 2.0 - 1.0).ceil() as i32
}

fn fill_grid(grid: &[bool], h: i32, w: i32) -> Vec<ThisIsMyBox> {
    // Descend fibonacci filling grid while possible
    let sizes = [100, 13, 8, 5, 3, 2, 1];
    let mut rng = thread_rng();

    // Since sizes can be even or odd, squares fill out approximate squares on even numbers
    // 1: x
    // 2: x o
    //    o o
    // 3: o o o
    //    o x o
    //    o o o
    // 5: o o o o o
    //    o o o o o
    //    o o x o o
    //    o o o o o
    //    o o o o o
    // 8: o o o o o o o o
    //    o o o o o o o o
    //    o o o o o o o o
    //    o o o x o o o o
    //    o o o o o o o o
    //    o o o o o o o o
    //    o o o o o o o o
    //    o o o o o o o o

    let mut pass = grid.to_vec();

    let len: i32 = pass.len() as i32;
    let mut order: Vec<i32> = (0..len).collect();
    order.shuffle(&mut rng);

    let mut boxes: Vec<ThisIsMyBox> = Vec::new();

    for size in sizes {
        // How much to subtract from x/y to get the top-left corner
        let offset = make_offset(size);
        println!("offset: {}", offset);
        for i in order.clone() {
            if !pass[i as usize] {
                let cx = i % w;
                let cy = i / h;
                // Spot is vacant, sweep square to see if candidate is eligible
                let mut valid = true;
                for y in 0..size {
                    for x in 0..size {
                        let dx = cx + x - offset;
                        let dy = cy + y - offset;

                        // Just ignore out-of-bounds coordinates
                        if dx < 0 || dy < 0 || dx >= w || dy >= h {
                            continue;
                        }

                        if pass[(dy * h + dx) as usize] {
                            valid = false;
                            break;
                        }
                    }
                }

                // If valid after sweep, mark the grid cells as occupied and push
                // the square to the vector of squares to draw
                if valid {
                    for y in 0..size {
                        for x in 0..size {
                            let dx = cx + x - offset;
                            let dy = cy + y - offset;

                            // Just ignore out-of-bounds coordinates
                            if dx < 0 || dy < 0 || dx >= w || dy >= h {
                                continue;
                            }

                            pass[(dy * h + dx) as usize] = true;
                        }
                    }
                    let rect = ThisIsMyBox {
                        x: cx - offset,
                        y: cy - offset,
                        size,
                    };
                    boxes.push(rect);
                    // print_grid(pass.clone(), h, w);
                }
            }
        }
    }

    boxes
}

fn print_grid(grid: Vec<bool>, h: i32, w: i32) {
    for row in 0..h {
        let s = (row * w) as usize;
        let e = s + w as usize;
        println!(
            "{}",
            &grid[s..e].iter().fold(String::new(), |s, b| {
                if *b {
                    s + "x"
                } else {
                    s + "-"
                }
            })
        );
    }
    println!("");
}
