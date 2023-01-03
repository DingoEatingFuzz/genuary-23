use nannou::prelude::*;
use nannou::rand::prelude::*;
use nannou::rand::thread_rng;

#[derive(Debug)]
struct Rect {
    x: i32,
    y: i32,
    size: i32,
}

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let boundary = app.window_rect();

    // Grid dimensions
    const w: i32 = 30;
    const h: i32 = 30;
    const size: usize = (w * h) as usize;

    // Grid occupancy
    let mut array: [bool; size] = [false; size];
    let boxes = fill_grid(&mut array, h, w);

    let draw = app.draw();
    draw.background().color(PLUM);

    // let sine = app.time.sin();
    // let slowersine = (app.time / 2.0).sin();

    // let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    // let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // draw.ellipse().color(STEELBLUE).x_y(x, y);
    println!("{}", boxes.len());

    for b in boxes.iter() {
        println!("Box: {:?}", b);
        draw.rect()
            .x_y(
                map_range(b.x, 0, w, boundary.left(), boundary.right()),
                map_range(b.y, 0, h, boundary.top(), boundary.bottom()),
            )
            .width(map_range(b.size, 0, w, boundary.left(), boundary.right()))
            .height(map_range(b.size, 0, h, boundary.top(), boundary.bottom()))
            .stroke(BLACK)
            .color(STEELBLUE);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn fill_grid(grid: &[bool], h: i32, w: i32) -> Vec<Rect> {
    // Descend fibonacci filling grid while possible
    let sizes = [8, 5, 3, 2, 1];
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

    let mut boxes: Vec<Rect> = Vec::new();

    for size in sizes {
        // How much to subtract from x/y to get the top-left corner
        let offset = ((size as f32) / 2.0 - 1.0).ceil() as i32;
        println!("offset: {}", offset);
        for i in order.clone() {
            if !pass[i as usize] {
                let cx = i % w;
                let cy = i / h;
                // Spot is vacant, sweep square to see if candidate is eligible
                let mut valid = true;
                for x in 0..size {
                    for y in 0..size {
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
                    for x in 0..size {
                        for y in 0..size {
                            let dx = cx + x - offset;
                            let dy = cy + y - offset;

                            // Just ignore out-of-bounds coordinates
                            if dx < 0 || dy < 0 || dx >= w || dy >= h {
                                continue;
                            }

                            pass[(dy * h + dx) as usize] = true;
                        }
                    }
                    let rect = Rect { x: cx, y: cy, size };
                    boxes.push(rect);
                }
            }
        }
    }

    boxes
}
