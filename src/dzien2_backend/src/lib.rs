use std::cell::RefCell;

thread_local! {
    static  WPISY: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String, surrname: i8) -> String {
    format!("Hello, {} {}!", name, surrname)
}

#[ic_cdk::update]
fn dodaj_wpis(wpis: String) {
    WPISY.with(|wpisy| {
        wpisy.borrow_mut().push(wpis)
    })
}