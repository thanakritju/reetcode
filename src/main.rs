mod fetcher;

fn main() {
    let result = fetcher::get_problems().unwrap();
    println!("{:?}", result);
}
