//! GPIO mock implementing digital::OutputPin and digital::InputPin
//! 
//! 

use std::cell::RefCell;
use std::format;
use std::string::String;

///
pub struct DigitalIOMock {
    name:  &'static str,
	data:  RefCell<Vec<bool>>,
    last:  RefCell<bool>,
    count: RefCell<usize>,
    enforce:bool,
}
/// # USAGE
///```
///  extern crate embedded_hal as hal;
///  extern crate embedded_hal_mock;
///  use hal::digital::{OutputPin,InputPin};
///  use embedded_hal_mock::gpio::DigitalIOMock;
/// 
///  let mut dummy_reset = DigitalIOMock::new("spi-rst",[false,true].to_vec());
///  dummy_reset.set_low();
///  // will fail internaly if called in incorrect order
///  dummy_reset.set_high();
///  	
/// ```
impl DigitalIOMock{
	/// Initiate in enforcing mode - the traditional mock mode
    pub fn new(name:&'static str, l:Vec<bool>)-> Self{
        DigitalIOMock{ name: name,
                       data: RefCell::new(l),
                       last: RefCell::new(false),
                       count: RefCell::new(0),
                       enforce: true }
    }
	/// Initiate in monitor mode, all set_high and set_low operations on a pin
	/// is stored and could be printed and used in a later refactoring of 
	/// existing code
    pub fn monitor(name:&'static str)-> Self{
        DigitalIOMock{ name: name,
                       data: RefCell::new(Vec::new()),
                       last: RefCell::new(false),
                       count: RefCell::new(0),
                       enforce: false }
    }
    fn inc(&self) -> usize {
        *self.count.borrow_mut() += 1;
        self.count.borrow().clone()
    }
	/// retrive pin settings collected with monitor()
    pub fn print(&self) -> String {
        let mut s = format !("new(\"{}\",[", self.name  );
        for i in self.data.borrow().iter() {
            s.push_str(&format!("{},",i));
        }
        s.push_str("].to_vec());");
        s
    }
}

impl hal::digital::OutputPin for DigitalIOMock {
	fn set_low(&mut self ) {
        if self.enforce {
            let num = self.inc();
            if num > self.data.borrow_mut().len() {
                assert!(false,"Vector {} out of bounds at {}",self.name, num)
            }
            let v = self.data.borrow();
            let refdata = v[num -1];
            assert!( refdata == false , "refdata {} unexpected at {}",self.name, num)
        }else{
            // Record tranactions for later analysis
            self.data.borrow_mut().push(false)
        }
	}
	fn set_high(&mut self) {
        if self.enforce {
            let num = self.inc();
            if num > self.data.borrow_mut().len() {
                assert!(false,"Vector {} out of bounds at {}",self.name, num)
            }
            let v = self.data.borrow();
            let refdata = v[num -1];
            assert!( refdata == true , "refdata {} unexpected at {}",self.name, num)
        }else{
            // Record tranactions for later analysis
            self.data.borrow_mut().push(true)
        }
    }
}

impl hal::digital::InputPin for DigitalIOMock {
	fn is_high(&self ) -> bool {
        let num = self.inc();
        if num > self.data.borrow_mut().len() {
            assert!(false, "Vector out of bounds: Returning last known state") ;
            return self.last.borrow().clone();
        }  
        let v = self.data.borrow();
        let refdata = v[num -1];
        self.last.replace(refdata);
        refdata
	}
	fn is_low(&self) -> bool{
	    ! self.is_high()
    }
}
#[cfg(test)]
mod test {
    use super::*;

    use hal::digital::{OutputPin,InputPin};

    #[test]
    fn test_gpio_mock_digitaloutput() {
		// Prepare digital pin with expected data
		let mut dummy_reset = DigitalIOMock::new("spi-rst",[false,true].to_vec());

		dummy_reset.set_low();
		// will fail internaly if called in incorrect order
		dummy_reset.set_high();
    }
	#[test]
	fn test_gpio_mock_digitalinput(){
		let dummy_gpio = DigitalIOMock::new("irq",[false,false,false,true,true].to_vec());
		let mut loopcnt = 0;
		while dummy_gpio.is_low(){
			loopcnt += 1;
		}
		assert!(loopcnt == 3);
		assert!(dummy_gpio.is_high());
	}	
	#[test]
	fn test_gpio_monitor_digitaloutput(){
		let mut dummy_gpio = DigitalIOMock::monitor("reset");
		dummy_gpio.set_low();
		dummy_gpio.set_high();
		println!("Captured data {}",dummy_gpio.print());
	}
}

