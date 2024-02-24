pub enum Cell {
    Alive,
    Dead,
}

pub fn print_options() {
    let options: Vec<Option<u32>> = vec![Some(0), None, Some(69)];
    let options: Vec<_> = options.iter().filter_map(Option::as_ref).collect();
    println!("{:?}", options);
}
