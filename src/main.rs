use gtm_api;

#[cfg(test)] mod test;

fn main() {
    gtm_api::rocket().launch();
}