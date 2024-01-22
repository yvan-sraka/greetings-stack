use hs_bindgen::*;

#[hs_bindgen(greetings :: CString -> IO ())]
fn greetings(name: &str) {
    println!("Hello, ${name}!");
}
