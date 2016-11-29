// Licensed: Apache 2.0
//
extern crate png_encode_mini;


// --------------
// Test Utilities

// Uses 'imagemagick' to convert and compare images ensures they can be loaded.
const USE_VALIDATE_EXTERNAL: bool = true;

fn image_generate_pixels(
    image_x: usize,
    image_y: usize,
) -> Vec<u8> {
    let len = image_x * image_y;
    let mut image: Vec<u8> = Vec::with_capacity(len * 4);

    for y in 0..image_y {
        for x in 0..image_x {
            let r = x as f64 / image_x as f64;
            let g = y as f64 / image_y as f64;
            let b = (1.0 - r) * g;

            image.extend(&[
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8,
                255_u8,
            ]);
        }
    }
    return image;
}

fn image_at_size(
    file: &String,
    image_x: usize,
    image_y: usize,
) {
    let image = image_generate_pixels(image_x, image_y);

    {
        let mut f = std::fs::File::create(file).unwrap();
        match png_encode_mini::write_rgba_from_u8(
            &mut f, &image[..],
            image_x as u32,
            image_y as u32,
        ) {
            Ok(_) => {
                // println!("Written image!")
            },
            Err(e) => {
                println!("Error {:?}", e)
            },
        }
    }
}

fn test_at_size(
    image_x: usize,
    image_y: usize,
) {
    let image_name_src = format!("test_{}_{}_src.png", image_x, image_y);
    let image_name_dst = format!("test_{}_{}_dst.png", image_x, image_y);
    let image_name_out = format!("test_{}_{}_out.png", image_x, image_y);
    image_at_size(&String::from(image_name_src.clone()), image_x, image_y);

    if USE_VALIDATE_EXTERNAL {
        use std::process::Command;

        let status = Command::new("convert")
            .arg(image_name_src.clone())
            .arg(image_name_dst.clone())
            .status().unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });
        assert_eq!(true, status.success());

        // Compare the images, ensure we get zero output
        // annoyingly we need to write out to a new image,
        // it seems theres no good way to avoid that.
        let output = Command::new("compare")
            .arg("-metric")
            // absolute error
            .arg("ae")
            .arg(image_name_src.clone())
            .arg(image_name_dst.clone())
            .arg(image_name_out.clone())
            .output().unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });
        // Ensure no difference: a single zero character
        assert_eq!(vec![48], output.stderr);

        // cleanup
        for f in &[
            image_name_src,
            image_name_dst,
            image_name_out,
        ] {
            ::std::fs::remove_file(f).expect("unable to remove file");
        }
    }
}


// -----
// Tests

macro_rules! test_at_size_gen {
    ($id:ident, $x:expr, $y:expr) => {
        #[test]
        fn $id() {
            test_at_size($x, $y);
        }
    }
}

test_at_size_gen!(test_1_1, 1, 1);
test_at_size_gen!(test_2_2, 2, 2);
test_at_size_gen!(test_2_3, 2, 3);
test_at_size_gen!(test_3_2, 3, 2);
test_at_size_gen!(test_4_4, 4, 4);
test_at_size_gen!(test_4_8, 4, 8);
test_at_size_gen!(test_8_8, 8, 8);
test_at_size_gen!(test_8_16, 8, 16);
test_at_size_gen!(test_16_8, 16, 8);
test_at_size_gen!(test_16_16, 16, 16);
test_at_size_gen!(test_16_32, 16, 32);
test_at_size_gen!(test_32_16, 32, 16);
test_at_size_gen!(test_32_32, 32, 32);
test_at_size_gen!(test_32_64, 32, 64);
test_at_size_gen!(test_64_32, 64, 32);
test_at_size_gen!(test_64_64, 64, 64);
test_at_size_gen!(test_64_128, 64, 128);
test_at_size_gen!(test_128_64, 128, 64);
test_at_size_gen!(test_128_128, 128, 128);
test_at_size_gen!(test_128_256, 128, 256);
test_at_size_gen!(test_256_128, 256, 128);
test_at_size_gen!(test_256_256, 256, 256);
test_at_size_gen!(test_256_512, 256, 512);
test_at_size_gen!(test_512_256, 512, 256);
test_at_size_gen!(test_512_512, 512, 512);
test_at_size_gen!(test_512_1024, 512, 1024);
test_at_size_gen!(test_1024_512, 1024, 512);
test_at_size_gen!(test_1024_1024, 1024, 1024);
test_at_size_gen!(test_1_13, 1, 13);
test_at_size_gen!(test_9_17, 9, 17);
test_at_size_gen!(test_21_13, 21, 13);
test_at_size_gen!(test_31_3, 31, 3);
test_at_size_gen!(test_99_21, 99, 21);
test_at_size_gen!(test_251_111, 251, 111);

