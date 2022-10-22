mod fetcher;

fn main() {
    let result = fetcher::lib::get_problems().unwrap();
    println!("{:?}", result);
}
