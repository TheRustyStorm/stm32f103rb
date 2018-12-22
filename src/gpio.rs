use core::ptr;


pub struct GPIO{
    BASE: u32,
}

pub enum GPIO_REGISTER{
    CRL,
    CRH,
    IDR,
    ODR,
    BSRR,
    BRR,
    LCKR
}

pub enum GPIO_PORT{
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl GPIO{
    pub fn new(port: GPIO_PORT) -> GPIO{
        GPIO{BASE: match port{
            GPIO_PORT::A => 0x4001_0800,
            GPIO_PORT::B => 0x4001_0C00,
            GPIO_PORT::C => 0x4001_1000,
            GPIO_PORT::D => 0x4001_1400,
            GPIO_PORT::E => 0x4001_1800,
            GPIO_PORT::F => 0x4001_1C00,
            GPIO_PORT::G => 0x4001_2000,
        }}
    }
    pub fn set(&self, register: GPIO_REGISTER, pattern: u32){
        let offset:u32 = match register{
            GPIO_REGISTER::CRL => 0x00,
            GPIO_REGISTER::CRH => 0x04,
            GPIO_REGISTER::IDR => 0x08,
            GPIO_REGISTER::ODR => 0x0C,
            GPIO_REGISTER::BSRR => 0x10,
            GPIO_REGISTER::BRR => 0x14,
            GPIO_REGISTER::LCKR => 0x18
        };
        unsafe{
            ptr::write_volatile((self.BASE + offset) as *mut u32, pattern);
        }   
    }

    pub fn set_bit(&self, register: GPIO_REGISTER, bit: u32){
        let offset:u32 = match register{
            GPIO_REGISTER::CRL => 0x00,
            GPIO_REGISTER::CRH => 0x04,
            GPIO_REGISTER::IDR => 0x08,
            GPIO_REGISTER::ODR => 0x0C,
            GPIO_REGISTER::BSRR => 0x10,
            GPIO_REGISTER::BRR => 0x14,
            GPIO_REGISTER::LCKR => 0x18
        };
        unsafe{
            let mut old:u32 = ptr::read((self.BASE + offset) as *mut u32);
            old |= 1u32<<bit;
            ptr::write((self.BASE + offset) as *mut u32, old);
        }   
    }
}