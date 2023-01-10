fn main() {
    // 0, 1, 3, 7 - hit
    // 2 - new combo
    // 4, 5, 6 - combo to skip
    let circle: u8 = 21;
    let slider: u8 = 6;
    let spinner: u8 = 8;
    fn new_combo(objtype: u8) -> bool {
        objtype & 4 != 0
    }

    fn color_skip(objtype: u8) -> u8 {
        (objtype >> 4) & 7
    }

    fn get_obj_type(objtype: i32) {
        match objtype & 139 {
            1 => println!("circle"),
            2 => println!("slider"),
            8 => println!("spinner"),
            128 => println!("HoldNote"),
            _ => println!("Invalid hit object type"),
        }
    }
}
