mod rand;

fn main() {
    let seed = 1880u64;

    let mut xo = rand::Xoshiro256::from_seed(seed);
    println!("{:?}", xo);
    for _x in 0..100 {
        let n = xo.rand();
        println!("Random: {:?}", n);
    }

    // Let's roll some dice!!
    let mut results = [0,0,0,0,0,0,0,0,0,0];
    let mut xo = rand::Xoshiro256::new();
    for _x in 0..1_000_000 {
        let n = xo.nroll(2, 4);
        results[n as usize] += 1;
    }
    println!("{:?}", results);
}
