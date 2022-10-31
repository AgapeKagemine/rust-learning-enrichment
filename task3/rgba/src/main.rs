fn hex_to_decimal(hex: &str) -> Result<u8, String>{
    match u8::from_str_radix(hex, 16) {
        Ok(decimal) => Ok(decimal),
        Err(e) => Err(e.to_string())
    }
}

fn hex_to_rgba(hex: &str) -> (u8, u8, u8, f32) {
    let (_, rgba) = hex.split_at(1); // Throw away # from the rest of the string - tuples

    let (red_green, blue_alpha) = rgba.split_at(4); // Split into 2 chunks (FFFF, FF00) - tuples

    // Split the first chunk (FFFF) and the second chunk (FF00) -> ( ( FF, FF ), ( FF, FF ) )
    let (red_green, blue_alpha) = (red_green.split_at(2), blue_alpha.split_at(2));

    let mut red: u8 = 0;
    let red_converted = hex_to_decimal(red_green.0);
    match red_converted {
        Ok(res) => red = res,
        Err(e) => println!("{}", e)
    }

    let mut green: u8 = 0;
    let green_converted = hex_to_decimal(red_green.1);
    match green_converted {
        Ok(res) => green = res,
        Err(e) => println!("{}", e)
    }

    let mut blue: u8 = 0;
    let blue_converted = hex_to_decimal(blue_alpha.0);
    match blue_converted {
        Ok(res) => blue = res,
        Err(e) => println!("{}", e)
    }

    let mut alpha: f32 = 0.0;
    let alpha_converted = hex_to_decimal(blue_alpha.1);
    match alpha_converted {
        Ok(res) => alpha = (1.0 / 255.0) * (res as f32),
        Err(e) => println!("{}", e)
    }
    
    (red, green, blue, alpha)
}

fn rgba_value((red, green, blue, alpha): (u8, u8, u8, f32)) {
    println!("R: {}, G: {}, B: {}, A: {}%", red, green, blue, alpha * 100.0);
}

fn main() {
    // FF - FF- FF - 00 => Red - Green - Blue - Alpha
    let hex = String::from("#FFFFFF00"); 
    rgba_value(hex_to_rgba(&hex));
}