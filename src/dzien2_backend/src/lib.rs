#[ic_cdk::query]
fn greet(name: String, surrname: i8) -> String {
    format!("Hello, {} {}!", name, surrname)
}
