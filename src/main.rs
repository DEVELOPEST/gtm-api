use gtm_api;

#[cfg(test)] mod tests;

fn main() {
    gtm_api::rocket().launch();
}