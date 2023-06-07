use rand::*;
use rand_chacha::ChaCha8Rng;

fn main() {
    println!("Hello, world!");
    let mut buffer = [false; 900];

    for s in 0..100 {
        let mut rng = ChaCha8Rng::seed_from_u64(s);
        for i in 0..900 {
            buffer[i] = rng.gen_ratio(1, 20);
        }
        rle(&mut buffer);
    }
}

fn rle(buffer: &mut [bool]) {
    let mut temp = [0u8; 900];
    let mut count = 0;
    let mut state = buffer[0];
    let mut bytes = 0;
    let mut t_count = 0;

    for i in 0..900 {
        let value = buffer[i];

        if value == true {
            t_count += 1;
        }

        if value == state {
            count += 1;
        }
        if value != state || i == 899 || count == 127 {
            temp[bytes] = state as u8;
            temp[bytes + 1] = count;
            state = value;
            count = 0;
            bytes += 2;
        }
    }
    // println!("count: {}", count);
    // println!("state: {}", state);
    println!("--------------------------");
    println!("press count: {}/900", t_count);
    println!("bytes: {}", bytes);
    println!("bytes / 2: {}", bytes / 2);
}
