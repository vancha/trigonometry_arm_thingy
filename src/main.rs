const len1:i32 = 400; //fixed length of link1 of robot arm
const len2:i32 = 410;//fixed length of link2 of robot arm
const objectpos:(f32,f32) = (500.0,500.0);//the object that will be pixed up has a fixed position now


//used to find the tird side of tringle when two sides and their enclosed angle are known.
fn lawOfCosines(a:f64,b:f64,c:f64) -> f64 {
    ((a*a + b*b - c*c)/ (2f64*a*b)).acos()
}

//returns the distance from (0,0) and x,y
fn distance(x:f64, y:f64)->f64 {
    (x*x + y*y).sqrt()
}

//returns the angles between (0,0) and point (x,y)
fn angles(x:f64, y:f64)->(f64,f64,f64,f64) {
    let dist = distance(x,y);
    let D1 = y.atan2(x);
    let D2 = lawOfCosines(dist,len1 as f64,len2 as f64);
    let A1 = D1 + D2;
    let A2 = lawOfCosines(len1 as f64, len2 as f64, dist);
    (A1,A2,D1,D2)
}

//gets the location of the end of the link of length len1, start position 0,0, and angle angle.
fn end_loc(angle:f64)->(f64,f64) {
    (len1 as f64 * angle.cos(),len1 as f64 * angle.sin())
}

//turns radians to degrees for coolness
fn deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        let pos = macroquad::input::mouse_position();
        if(distance(pos.0 as f64, pos.1 as f64) < len1 as f64 + len2 as f64){
        clear_background(BLACK);//sets the blackground to a solid black color
        let (a1,a2,d1,d2) = angles(pos.0 as f64, pos.1 as f64);//tries to get the angles of a line pointing towards the mouse cursor.
        let (x2,y2) = end_loc(d1 + d2);//gets the end position of that line
        draw_line(0.0,0.0, x2 as f32,y2 as f32,15.0,BLUE);//draws that line
        draw_line(x2 as f32,y2 as f32,pos.0 as f32,pos.1 as f32,15.0,BLUE);//draws the second line that reaches towards the desired position
        }
        next_frame().await
    }
}
