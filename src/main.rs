#![no_std]
#![no_main]

use ch32_hal::{self as hal, timer};
use hal::pac::{self, gpio::vals};
use hal::delay::Delay;
use hal::gpio::{Level, Output, OutputOpenDrain};
use hal::timer::simple_pwm::{PwmPin, SimplePwm};
use hal::timer::low_level::CountingMode;
use hal::time::Hertz;



#[qingke_rt::entry]
fn main() -> ! {
//     hal::debug::SDIPrint::enable();
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSE;
    let p = hal::init(config);
    let mut delay = Delay;

    let pin_config = hal::Config::default();

    let mut sync = PwmPin::new_ch4::<0>(p.PC4);
    let mut pwm = SimplePwm::new(p.TIM1, None, None, None, Some(sync), Hertz::hz(15750), CountingMode::EdgeAlignedUp);

    pac::GPIO(2).cfglr().modify(|w| w.set_cnf(4, vals::Cnf::AF_OPEN_DRAIN_OUT));
    
    pwm.enable(timer::Channel::Ch4);

    loop {
        // start of line

    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(debug_assertions)]
    {
        let _ = hal::println!("\n\n\n{}", info);
    }
    loop {}
}

