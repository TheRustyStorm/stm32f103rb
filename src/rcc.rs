use core::ptr;

pub struct RCC{
    BASE: u32,
}

pub enum RCC_REGISTER{
    CR,
    CFGR,
    CIR,
    APB2RSTR,
    APB1RSTR,
    AHBENR,
    APB2ENR,
    APB1ENR,
    BDCR,
    CSR
}

impl RCC{
    pub fn new() -> RCC{
        RCC{BASE: 0x4002_1000}
    }
    pub fn set(&self, register: RCC_REGISTER, pattern: u32){
        let offset:u32 = match register{
            RCC_REGISTER::CR => 0x00,
            RCC_REGISTER::CFGR => 0x04,
            RCC_REGISTER::CIR => 0x08,
            RCC_REGISTER::APB2RSTR => 0x0C,
            RCC_REGISTER::APB1RSTR => 0x10,
            RCC_REGISTER::AHBENR => 0x14,
            RCC_REGISTER::APB2ENR => 0x18,
            RCC_REGISTER::APB1ENR => 0x1C,
            RCC_REGISTER::BDCR => 0x20,
            RCC_REGISTER::CSR => 0x24,
        };
        unsafe{
            ptr::write_volatile((self.BASE + offset) as *mut u32, pattern);
        }   
    }

    pub fn set_bit(&self, register: RCC_REGISTER, bit: u32){
        let offset:u32 = match register{
            RCC_REGISTER::CR => 0x00,
            RCC_REGISTER::CFGR => 0x04,
            RCC_REGISTER::CIR => 0x08,
            RCC_REGISTER::APB2RSTR => 0x0C,
            RCC_REGISTER::APB1RSTR => 0x10,
            RCC_REGISTER::AHBENR => 0x14,
            RCC_REGISTER::APB2ENR => 0x18,
            RCC_REGISTER::APB1ENR => 0x1C,
            RCC_REGISTER::BDCR => 0x20,
            RCC_REGISTER::CSR => 0x24,
        };
        unsafe{
            let mut old:u32 = ptr::read_volatile((self.BASE + offset) as *mut u32);
            old |= 1u32<<bit;
            ptr::write_volatile((self.BASE + offset) as *mut u32, old);
        }   
    }
}

