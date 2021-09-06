const len1:i32 = 400;
const len2:i32 = 410;

const objectpos:(f32,f32) = (500.0,500.0);
fn lawOfCosines(a:f64,b:f64,c:f64) -> f64 {
    ((a*a + b*b - c*c)/ (2f64*a*b)).acos()
}

fn distance(x:f64, y:f64)->f64 {
    (x*x + y*y).sqrt()
}

fn angles(x:f64, y:f64)->(f64,f64,f64,f64) {
    let dist = distance(x,y);
    let D1 = y.atan2(x);
    let D2 = lawOfCosines(dist,len1 as f64,len2 as f64);
    let A1 = D1 + D2;
    let A2 = lawOfCosines(len1 as f64, len2 as f64, dist);
    (A1,A2,D1,D2)
}
fn deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}
/*fn main() {
    println!("moving to (5,5):");
    let (x,y)=(5f64,5f64);
    let (a1,a2) = angles(x,y);
    println!("x={},y={}: A1={} ({}), A2={} ({})\n",x,y,a1,deg(a1),a2,deg(a2));
}*/

fn end_loc(angle:f64)->(f64,f64) {
    //(len1 as f64 * angle.cos(), len1 as f64 * angle.sin())
    (len1 as f64 * angle.cos(), len1 as f64 * angle.sin())
}
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(BLACK);
        let pos = macroquad::input::mouse_position();
        //draw_line(40.0, 40.0, pos.0, pos.1, 15.0, BLUE);
        let (a1,a2,d1,d2) = angles(pos.0 as f64, pos.1 as f64);
        let (x2,y2) = end_loc(d1);
        draw_line(0.0,0.0, x2 as f32,y2 as f32,15.0,BLUE);
        draw_circle(objectpos.0, objectpos.1,15.0, YELLOW);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
