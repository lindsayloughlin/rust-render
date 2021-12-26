extern crate image;
extern crate num;

use image::{GenericImageView, ImageBuffer, RgbImage, RgbaImage, Rgb, save_buffer_with_format};

struct Point {
    x: i32,
    y: i32,
}


fn Interpolate(i0 : i32, d0: i32, i1: i32, d1 : i32 ) -> Vec<i32> {

    if i0 == i1{
        return vec![d0]
    }
    let mut valOut = Vec::new();
    let a = (d1 - d0) / (i1 - i0);
    let mut d = d0;
    let mut x  = i0;
    while x <= i1 {        
        valOut.push(d);
        d += a;
        x = x + 1;
    }
    return valOut

}

// fn Swap(p0 : Point, p1 : Point) {
//     let swap = p0;

// }
// fn GenerateFractal() {

//     let imgx = 100;
//     let imgy = 100;

//     let scalex = 3.0 / imgx as f32;
//     let scaley = 3.0 / imgy as f32;

//     // Create a new ImgBuf with width: imgx and height: imgy
//     let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

//     // Iterate over the coordinates and pixels of the image
//     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//         let r = (0.3 * x as f32) as u8;
//         let b = (0.3 * y as f32) as u8;
//         *pixel = image::Rgb([r, 0, b]);
//     }

//     // A redundant loop to demonstrate reading image data
//     for x in 0..imgx {
//         for y in 0..imgy {
//             let cx = y as f32 * scalex - 1.5;
//             let cy = x as f32 * scaley - 1.5;

//             let c = num::complex::Complex::new(-0.4, 0.6);
//             let mut z = num::complex::Complex::new(cx, cy);

//             let mut i = 0;
//             while i < 255 && z.norm() <= 2.0 {
//                 z = z * z + c;
//                 i += 1;
//             }

//             let pixel = imgbuf.get_pixel_mut(x, y);
//             let image::Rgb(data) = *pixel;
//             *pixel = image::Rgb([data[0], i as u8, data[2]]);
//         }
//     }
//     // let imageVec = imgbuf.into_vec();
//     // for item in imageVec {
//     //     println!("{}", item)
//     // }
//     // Save the image as “fractal.png”, the format is deduced from the path
//     // imgbuf.save("fractal.png").unwrap();

// }

fn DrawIntoImage() {

    let green = Rgb([0 as u8, 255 as u8, 0 as u8]);
    let mut img = RgbImage::new(32, 32);
    img.put_pixel(10, 10, green);

}

fn DrawLine(pa : Point, pb : Point, img : &mut RgbImage) {
    let mut p0 = pa;
    let mut p1 = pb;
    let dx = p1.x - p0.x ;
    let dy = p1.y - p0.y ;
    let white = image::Rgb([255 as u8,255 as u8,255 as u8]);
    if dx.abs() > dy.abs() {

        if dx < 0 {
            let swap = p0;
            p0 = p1 ;
            p1 = swap;
        }
        let ys = Interpolate(p0.x, p0.y, p1.x, p1.y);
        let mut x = p0.x ;
 //       let mut *pixel = & image::Rgb([100 as u8,100 as u8,100 as u8]);
        while x <= p1.x {
            let ybit = ys[(x-p0.x) as usize];            
            img.put_pixel(x as u32, ybit as u32, white);
            x = x + 1;
        }
    } else {
        if dy < 0 {
            let mut swap = p0;
            p0 = p1;
            p1 = swap;
        }
        let xs = Interpolate(p0.y, p0.x, p1.y,p1.x);
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
    let mut img = RgbImage::new(800,800);
    //GenerateFractal();
    DrawLine( Point { x: 100, y: 100 }, Point { x: 200, y: 200 },&mut img);
    img.save("lineimage.png");
}

