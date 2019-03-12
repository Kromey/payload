mod rand;

fn main() {
    let seed = 1880u64;

    let mut xo = rand::Rand::from_seed(seed);
    println!("{:?}", xo);
    for _x in 0..100 {
        let n = xo.rand_u64();
        println!("Random: {:?}", n);
    }

    // Let's roll some dice!!
    let mut results = [0,0,0,0,0,0,0,0,0,0];
    let mut xo = rand::Rand::new();
    for _x in 0..1_000_000 {
        let n = xo.roll_ndx(2, 4);
        results[n as usize] += 1;
    }
    println!("{:?}", results);
}
