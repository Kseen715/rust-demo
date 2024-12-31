#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
// include!(concat!(env!("OUT_DIR"), "/wiringPi.rs"));
include!(concat!(env!("OUT_DIR"), "/gpio.rs"));

fn main() {
    println!("Hello, world!");

    unsafe {
        wiringPiSetup();

        let gpio_num: u8 = getGpioNum();
        if 0xff == gpio_num {
            println!("wiringPiSetup failed");
            return;
        }
        println!("wiringPiSetup success, gpio_num: {}", gpio_num);

        for i in 0..gpio_num {
            pinMode(i.into(), OUTPUT.try_into().unwrap());
            println!("[INFO] Set GPIO {} as OUTPUT", i);
        }

        loop {
            println!("[INFO] Start to blink all GPIOs");
            for i in 0..gpio_num {
                digitalWrite(i.into(), HIGH.try_into().unwrap());
            }
            println!("[INFO] All GPIOs are HIGH");
            delay(5000);
            for i in 0..gpio_num {
                digitalWrite(i.into(), LOW.try_into().unwrap());
            }
            println!("[INFO] All GPIOs are LOW");
            delay(5000);
        }
    }
}
