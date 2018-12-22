use core::ptr;

pub struct TIM6_7{
    BASE: u32,
}

pub enum TIM6_7_REGISTER{
    CR1,
    CR2,
    DIER,
    SR,
    EGR,
    CNT,
    PSC,
    ARR,
}

pub enum TIM6_7_TYPE{
    TIM6,
    TIM7,
}

impl TIM6_7{
    pub fn new(t:TIM6_7_TYPE) -> TIM6_7{
        TIM6_7{BASE: match t{
            TIM6_7_TYPE::TIM6 => 0x4000_1000,
            TIM6_7_TYPE::TIM7 => 0x4000_1400,
        }}
    }
    pub fn set(&self, register: TIM6_7_REGISTER, pattern: u32){
        let offset:u32 = match register{
            TIM6_7_REGISTER::CR1 => 0x00,
            TIM6_7_REGISTER::CR2 => 0x04,
            TIM6_7_REGISTER::DIER => 0x0C,
            TIM6_7_REGISTER::SR => 0x10,
            TIM6_7_REGISTER::EGR => 0x14,
            TIM6_7_REGISTER::CNT => 0x24,
            TIM6_7_REGISTER::PSC => 0x28,
            TIM6_7_REGISTER::ARR => 0x2C
        };
        unsafe{
            ptr::write((self.BASE + offset) as *mut u32, pattern);
        }   
    }

    pub fn set_bit(&self, register: TIM6_7_REGISTER, bit: u32){
        let offset:u32 = match register{
            TIM6_7_REGISTER::CR1 => 0x00,
            TIM6_7_REGISTER::CR2 => 0x04,
            TIM6_7_REGISTER::DIER => 0x0C,
            TIM6_7_REGISTER::SR => 0x10,
            TIM6_7_REGISTER::EGR => 0x14,
            TIM6_7_REGISTER::CNT => 0x24,
            TIM6_7_REGISTER::PSC => 0x28,
            TIM6_7_REGISTER::ARR => 0x2C
        };
        
        unsafe{
            let mut old:u32 = ptr::read((self.BASE + offset) as *mut u32);
            old |= 1u32<<bit;
            ptr::write((self.BASE + offset) as *mut u32, old);
        } 
    }
    
}




