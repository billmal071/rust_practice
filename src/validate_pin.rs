/***
* ATM machines allow 4 or 6 digit PIN codes and PIN codes cannot contain anything but exactly 4 digits or exactly 6 digits.

If the function is passed a valid PIN string, return true, else return false.
*
*/
pub fn validate_pin(pin: &str) -> bool {
    let good_length = if pin.len() == 4 || pin.len() == 6 {
        true
    } else {
        false
    };
    let is_valid = pin.chars().all(|c| c.is_digit(10));
    // if pin.len() == 4 || pin.len() == 6 {
    //     for c in pin.chars() {
    //         if !c.is_digit(10) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    // false
    good_length && is_valid
}
