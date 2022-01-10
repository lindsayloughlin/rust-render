extern crate image;
extern crate num;

use image::{save_buffer_with_format, GenericImageView, ImageBuffer, Rgb, RgbImage, RgbaImage};

//use render_helper::{interpolate};
//use math::round;
///import render_helper;

mod render_functions;

#[derive(Copy, Clone)]
struct Colour {
    r: i32,
    g: i32,
    b: i32,
}

#[derive(Copy, Clone)]
struct ColouredPoint {
    point: Point,
    colour: Colour,
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

//fn interpolateColour(colourStart: color, d0: i32, colourEnd: colour, d1: i32) -> Vec<Colour> {

//    let mut val_out = Vec:new();
//   let ra =
//}

// fn swap_if_required(p0 : &Point, p1 : &Point ) -> (Point, Point) {
//     if p1.y > p0.y {
//         return (*p1, *p0)
//     }

//     return (p0, p1)
// }


fn modify_colour(multi : i32) -> image::Rgb<u8> {
    let notwhite = image::Rgb([multi as u8, 0 as u8, 255 as u8]);
    return notwhite;
}


fn draw_triangle_shaded(points: [Point; 3], img: &mut RgbImage) {
    let mut p0 = &points[0];
    let mut p1 = &points[1];
    let mut p2 = &points[2];

    // let (p0, p1)  = swap_if_required(p0, p1);
    if p1.y < p0.y {
        let swap = p0;
        p0 = p1;
        p1 = swap;
    }

    if p2.y < p0.y {
        let swap = p0;
        p0 = p2;
        p2 = swap
    }

    if p2.y < p1.y {
        let swap = p1;
        p1 = p2;
        p2 = swap;
    }

    let mut x01 = render_functions::interpolate(p0.y, p0.x, p1.y, p1.x);

    let mut h01 = render_functions::interpolate(p0.y, 0, p1.y, 245);

    let x12 = render_functions::interpolate(p1.y, p1.x, p2.y, p2.x);
    let mut h12 = render_functions::interpolate(p1.y, 245, p2.y, 0);

    let x02 = render_functions::interpolate(p0.y, p0.x, p2.y, p2.x);
    // remover the last of x01. as this value should appear in the x12 interpolate vectory.
    let mut h02 = render_functions::interpolate(p0.y, 0, p2.y, 100);

    x01.remove(x01.len() - 1);

    let mut x012: Vec<i32> = vec![];
    x012.extend_from_slice(&x01[..]);
    x012.extend_from_slice(&x12[..]);

    let mut h012: Vec<i32> = vec![];
    h012.extend_from_slice(&h01[..]);
    h012.extend_from_slice(&h12[..]);

    let m = (x012.len() as f32 / 2.0).floor();
    let x_left;
    let x_right;
    let h_left;
    let h_right;

    if &x02[m as usize] < &x012[m as usize] {
        x_left = x02;
        x_right = x012;
        h_left = h02;
        h_right = h12;
    } else {
        x_left = x012;
        x_right = x02;
        h_left = h012;
        h_right = h02;
    }

    for y in p0.y..p2.y {

        let ydiff = (y - p0.y) as usize;

        // Possibly truncate to zero.

        let x_l = x_left[ydiff];
        let x_r = x_right[ydiff];

        let h_segment = render_functions::interpolate(x_l, h_left[ ydiff ], x_r , h_right[ydiff]);

        for x in x_left[(y - p0.y) as usize]..x_right[(y - p0.y) as usize] {
            //img.put_pixel(x as u32, y as u32, notwhite)

            let segamount = h_segment.get( (x - x_l) as usize).unwrap();
            let color = modify_colour(*segamount);
            img.put_pixel(x as u32,y as u32,color)
        }
    }
}

fn draw_triangle_filled(points: [Point; 3], img: &mut RgbImage) {
    let mut p0 = &points[0];
    let mut p1 = &points[1];
    let mut p2 = &points[2];

    // let (p0, p1)  = swap_if_required(p0, p1);
    if p1.y < p0.y {
        let swap = p0;
        p0 = p1;
        p1 = swap;
    }

    if p2.y < p0.y {
        let swap = p0;
        p0 = p2;
        p2 = swap
    }

    if p2.y < p1.y {
        let swap = p1;
        p1 = p2;
        p2 = swap;
    }

    let mut x01 = render_functions::interpolate(p0.y, p0.x, p1.y, p1.x);
    let x12 = render_functions::interpolate(p1.y, p1.x, p2.y, p2.x);
    let x02 = render_functions::interpolate(p0.y, p0.x, p2.y, p2.x);
    // remover the last of x01. as this value should appear in the x12 interpolate vectory.
    x01.remove(x01.len() - 1);

    let mut x012: Vec<i32> = vec![];
    x012.extend_from_slice(&x01[..]);
    x012.extend_from_slice(&x12[..]);

    let m = (x012.len() as f32 / 2.0).floor();
    let x_left;
    let x_right;

    if &x02[m as usize] < &x012[m as usize] {
        x_left = x02;
        x_right = x012;
    } else {
        x_left = x012;
        x_right = x02;
    }

    let white = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
    for y in p0.y..p2.y {
        for x in x_left[(y - p0.y) as usize]..x_right[(y - p0.y) as usize] {
            img.put_pixel(x as u32, y as u32, white)
        }
    }
}

fn draw_triangle_wireframe(points: [Point; 3], img: &mut RgbImage) {
    for x in 0..3 {
        println!("{}", x);
        let a = &points[x];
        let b = &points[(x + 1) % (points.len())];
        draw_line(a, b, img);
    }
}

fn draw_line(pa: &Point, pb: &Point, img: &mut RgbImage) {
    let mut p0 = pa;
    let mut p1 = pb;
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    let white = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
    if dx.abs() > dy.abs() {
        if dx < 0 {
            let swap = p0;
            p0 = p1;
            p1 = swap;
        }
        let ys = render_functions::interpolate(p0.x, p0.y, p1.x, p1.y);
        let mut x = p0.x;
        //       let mut *pixel = & image::Rgb([100 as u8,100 as u8,100 as u8]);
        while x <= p1.x {
            let ybit = ys[(x - p0.x) as usize];
            img.put_pixel(x as u32, ybit as u32, white);
            x = x + 1;
        }
    } else {
        if dy < 0 {
            let swap = p0;
            p0 = p1;
            p1 = swap;
        }
        let xs = render_functions::interpolate(p0.y, p0.x, p1.y, p1.x);
        let mut y = p0.y;
        while y <= p1.y {
            let xbit = xs[(y - p0.y) as usize];
            img.put_pixel(xbit as u32, y as u32, white);
            y = y + 1
        }
    }
}

fn main() {
    //     let mut image = ImageBuffer::new(800, 800);
    //     *image.get_pixel_mut(5, 5) = image::Rgb([255,255,255]);

    //     // image.save("output.png")();

    //     println!("Hello, world!");
    //  //   let img: RgbaImage = ImageBuffer::new(512, 512);
    //     let mut img = ImageBuffer::from_fn(512, 512, |x,y| {
    //         if x % 2 == 0 {
    //             image::Luma([0u8])
    //         } else {
    //             image::Luma([255u8])
    //         }

    //     });

    //    let img = ImageBuffer::new(512, 512);
    let mut img = RgbImage::new(800, 800);
    //GenerateFractal();
    //draw_line( &Point { x: 100, y: 100 }, &Point { x: 200, y: 200 },&mut img);
    let points = [
        Point { x: 150, y: 150 },
        Point { x: 450, y: 450 },
        Point { x: 175, y: 250 },
    ];
    let lower_points = [
        Point { x: 150, y: 50 },
        Point { x: 450, y: 350 },
        Point { x: 175, y: 150 },
    ];

    let solid_points = [
        Point { x: 150, y: 250 },
        Point { x: 450, y: 550 },
        Point { x: 175, y: 350 },
    ];

    let shaded_points = [
        
        Point { x: 150, y: 350 },
        Point { x: 450, y: 650 },
        Point { x: 175, y: 450 },
    ];

    draw_triangle_wireframe(points, &mut img);
    draw_triangle_wireframe(lower_points, &mut img);
    draw_triangle_filled(solid_points, &mut img);
    draw_triangle_shaded(shaded_points, &mut img);
    let _result = img.save("lineimage.png");
}
