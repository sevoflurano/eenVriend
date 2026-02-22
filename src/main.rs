mod functions;

fn main() {
    loop {
        functions::greet_user(String::new());
        functions::user_option(String::new());
    }
}
