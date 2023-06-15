use joycon_typer::controller::Controller;

fn main() {
    loop{
        Controller::record_new_sample();
    }
}
