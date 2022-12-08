use std::cmp::Ordering;
use ys_simple_db::DB;

fn main() {
    let mut db = DB::from_file("examples/in.csv", 1, ",");
    db.insert(&["c"], &["1","0.1"]);
    db.to_file("examples/out.csv", ",");

    let mut vec = db.to_vec();
    vec.sort_by(|a, b|
        if a[1].parse::<f32>().unwrap() < b[1].parse().unwrap() {Ordering::Less}
        else if a[1].parse::<f32>().unwrap() > b[1].parse().unwrap() {Ordering::Greater}
        else {Ordering::Equal}
    );
    println!("{:?}", vec);
}