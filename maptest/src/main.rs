use payload::ship;


fn main() {
    let sector = ship::Sector::new();

    println!("{:?}", sector);

    sector.print();
}
