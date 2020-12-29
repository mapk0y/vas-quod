

fn check_collect_vec() {
    let c = "foo bar fizz bazz";
    let cb = c.split(" ");
    let cv = cb.collect::<Vec<&str>>();
    println!("{:?}", cv);
}

fn main() {
    check_collect_vec();
}