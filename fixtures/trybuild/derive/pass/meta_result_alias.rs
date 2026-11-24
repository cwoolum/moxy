#[allow(dead_code)]
type Result<T> = std::result::Result<T, String>;

#[derive(moxy::FromMeta)]
struct Args {
    flag: bool,
}

fn main() {
    let meta: moxy::ast::Meta = moxy::parse!("args(flag)").unwrap();
    assert!(meta.parse::<Args>().unwrap().flag);
}
