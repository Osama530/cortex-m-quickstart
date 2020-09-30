
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
            
            let mut gpio_e = dp.GPIOE.split(&mut rcc.ahb);
            let mut led_01 = gpio_e.pe9.into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
            let mut led_02 = gpio_e.pe10.into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
            let mut led_03 = gpio_e.pe11.into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
            let mut led_04 = gpio_e.pe12.into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
            let mut led_05 = gpio_e.pe13.into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
            let mut led_06 = gpio_e.pe14.into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
            let mut led_07 = gpio_e.pe15.into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);
            let mut led_08 = gpio_e.pe8.into_push_pull_output(&mut gpio_e.moder, &mut gpio_e.otyper);

            led_01.set_low().unwrap();
            led_02.set_low().unwrap();
            led_03.set_low().unwrap();
            led_04.set_low().unwrap();
            led_05.set_low().unwrap();
            led_06.set_low().unwrap();
            led_07.set_low().unwrap();
            led_08.set_low().unwrap();
        
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
                
                //let maped_value = map(0, 4092, 4);
                //hprintln!("maped value = {}", maped_value).ok();
                let maped_value = map(adc1_in1_data, 0, 4096, 0, 8);
                hprintln!("mapped value reads {}", &maped_value).ok();
                match maped_value {
                    1 => {
                        led_01.set_high().unwrap();
                        led_02.set_low().unwrap();
                        led_03.set_low().unwrap();
                        led_04.set_low().unwrap();
                        led_05.set_low().unwrap();
                        led_06.set_low().unwrap();
                        led_07.set_low().unwrap();
                        led_08.set_low().unwrap();
                    },
                    
                    2 => {
                    led_01.set_high().unwrap();
                    led_02.set_high().unwrap();
                    led_03.set_low().unwrap();
                    led_04.set_low().unwrap();
                    led_05.set_low().unwrap();
                    led_06.set_low().unwrap();
                    led_07.set_low().unwrap();
                    led_08.set_low().unwrap();
                    },

                    3 => {
                    led_01.set_high().unwrap();
                    led_02.set_high().unwrap();
                    led_03.set_high().unwrap();
                    led_04.set_low().unwrap();
                    led_05.set_low().unwrap();
                    led_06.set_low().unwrap();
                    led_07.set_low().unwrap();
                    led_08.set_low().unwrap();
                    },

                    4 => {
                    led_01.set_high().unwrap();
                    led_02.set_high().unwrap();
                    led_03.set_high().unwrap();
                    led_04.set_high().unwrap();
                    led_05.set_low().unwrap();
                    led_06.set_low().unwrap();
                    led_07.set_low().unwrap();
                    led_08.set_low().unwrap();
                    },
                    
                    5 => {
                    led_01.set_high().unwrap();
                    led_02.set_high().unwrap();
                    led_03.set_high().unwrap();
                    led_04.set_high().unwrap();
                    led_05.set_high().unwrap();
                    led_06.set_low().unwrap();
                    led_07.set_low().unwrap();
                    led_08.set_low().unwrap();
                    },
                    
                    6 => {
                    led_01.set_high().unwrap();
                    led_02.set_high().unwrap();
                    led_03.set_high().unwrap();
                    led_04.set_high().unwrap();
                    led_05.set_high().unwrap();
                    led_06.set_high().unwrap();
                    led_07.set_low().unwrap();
                    led_08.set_low().unwrap();
                    },
                    
                    7 => {
                    led_01.set_high().unwrap();
                    led_02.set_high().unwrap();
                    led_03.set_high().unwrap();
                    led_04.set_high().unwrap();
                    led_05.set_high().unwrap();
                    led_06.set_high().unwrap();
                    led_07.set_high().unwrap();
                    led_08.set_low().unwrap();
                    },
                    
                    8 => {
                    led_01.set_high().unwrap();
                    led_02.set_high().unwrap();
                    led_03.set_high().unwrap();
                    led_04.set_high().unwrap();
                    led_05.set_high().unwrap();
                    led_06.set_high().unwrap();
                    led_07.set_high().unwrap();
                    led_08.set_high().unwrap();
                    },
                    _ => {
                    led_01.set_low().unwrap();
                    led_02.set_low().unwrap();
                    led_03.set_low().unwrap();
                    led_04.set_low().unwrap();
                    led_05.set_low().unwrap();
                    led_06.set_low().unwrap();
                    led_07.set_low().unwrap();
                    led_08.set_low().unwrap();
                    }, 
                

            }
        }
    }

fn map(value: u16, lower_bound: u16, upper_bound: u16, lower_constrain: u16, upper_constrain: u16)-> u16 {
    let diff_bound = upper_bound - lower_bound;
    let diff_constrain = upper_constrain - lower_constrain;

    let result = (diff_constrain as f32 / diff_bound as f32) * value as f32;
    result as u16
}
