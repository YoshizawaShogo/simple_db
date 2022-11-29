use ys_simple_db::DB;

fn main() {
    let mut db = DB::from_file("examples/in.csv", 1, ",");
    db.insert(["c"].to_vec(), ["1","2"].to_vec());
    db.to_file("examples/out.csv", ",");
    println!("{:?}", db.get(&["c"].to_vec()));
}