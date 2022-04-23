#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    hci_bildung::build_rocket()
}
