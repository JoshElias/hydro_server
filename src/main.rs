use hydro_services::water_pump;

fn main() {
    //camera::take_picture();
    water_pump::start_pump().unwrap();
}
