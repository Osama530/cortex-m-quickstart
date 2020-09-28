// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// use panic_halt as _;

// use core::fmt::Write;

// use cortex_m::peripheral::syst::SystClkSource;
// use cortex_m_rt::{entry, exception};
// use cortex_m_semihosting::{
//     debug,
//     hio::{self, HStdout},
// };

// #[entry]
// fn main() -> ! {
//     let p = cortex_m::Peripherals::take().unwrap();
//     let mut syst = p.SYST;

//     // configures the system timer to trigger a SysTick exception every second
//     syst.set_clock_source(SystClkSource::Core);
//     // this is configured for the LM3S6965 which has a default CPU clock of 12 MHz
//     syst.set_reload(12_000_000);
//     syst.clear_current();
//     syst.enable_counter();
//     syst.enable_interrupt();

//     loop {}
// }

// #[exception]
// fn SysTick() {
//     static mut COUNT: u32 = 0;
//     static mut STDOUT: Option<HStdout> = None;

//     *COUNT += 1;

//     // Lazy initialization
//     if STDOUT.is_none() {
//         *STDOUT = hio::hstdout().ok();
//     }

//     if let Some(hstdout) = STDOUT.as_mut() {
//         write!(hstdout, "{}", *COUNT).ok();
//     }

//     // IMPORTANT omit this `if` block if running on real hardware or your
//     // debugger will end in an inconsistent state
//     // if *COUNT == 9 {
//     //     // This will terminate the QEMU process
//     //     debug::exit(debug::EXIT_SUCCESS);
//     // }
// }

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;
//peripharal access crate
//use stm32f3::stm32f303;
//HAL support crate
use stm32f3xx_hal::{pac, prelude::*,adc};


#[entry]
fn main() -> ! {
    //let peripherals = stm32f303::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
    
     	    // set up adc1
             let mut adc1 = adc::Adc::adc1(
                dp.ADC1, // The ADC we are going to control
                // The following is only needed to make sure the clock signal for the ADC is set up
                // correctly.
                &mut dp.ADC1_2,
                &mut rcc.ahb,
                adc::CkMode::default(),
                clocks,
            );
        
            // Set up pin PA0 as analog pin.
            // This pin is connected to the user button on the stm32f3discovery board.
            let mut gpio_a = dp.GPIOA.split(&mut rcc.ahb);
            let mut adc1_in1_pin = gpio_a.pa0.into_analog(&mut gpio_a.moder, &mut gpio_a.pupdr);
        
            // Be aware that the values in the table below depend on the input of VREF.
            // To have a stable VREF input, put a condensator and a volt limiting diode in front of it.
            //
            // Also know that integer division and the ADC hardware unit always round down.
            // To make up for those errors, see this forum entry:
            // [https://forum.allaboutcircuits.com/threads/why-adc-1024-is-correct-and-adc-1023-is-just-plain-wrong.80018/]
            hprintln!("
            The ADC has a 12 bit resolution, i.e. if your reference Value is 3V:
                approx. ADC value | approx. volt value
                ==================+===================
                                0 |        0 mV
                             2048 |     1500 mV
                             4095 |     3000 mV
        
            If you are using a STM32F3Discovery, PA0 is connected to the User Button.
            Pressing it should connect the user Button to to HIGH and the value should change from 0 to 4095.
            ").expect("Error using hprintln.");

    
            loop {
                let adc1_in1_data: u16 = adc1.read(&mut adc1_in1_pin).expect("Error reading adc1.");
                hprintln!("PA0 reads {}", adc1_in1_data).ok();
            }
}

