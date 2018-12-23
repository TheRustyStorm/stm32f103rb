# STM32F103RB

This projects was made with cortex-m-quickstart

![stm32 with LED on](https://github.com/TheSovietStorm/stm32f103rb/blob/master/img.png "stm32 with LED on")

# What is this?

I want to try to create a library to control my stm32 board in a rusty way. Currently it is able to access gpio and change the state of the on board LED, so that should give you a clue on how far this project currently is.

The next goal is to write a Timer abstraction for a delay function.

# What do you need if you want to try this project?

A stm32f103rb nucleo board and rust beta/nightly.

# How to run?

One terminal:
```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f1x.cfg
```
Other terminal:
```
cargo run
```
This will open a gdb session to debug your project.

# GPIO

The first thing to do is making the on board LED blink.
If you've worked with Arduino before, on avr it looks like this:
```
DDRB |= (1<<PB5);
PORTB |= (1<<PB5);
```
The on Board LED on an Arduino Uno is connected to Pin PB5, which is the fifth gpio pin on Port B.
In the first line we configure the data direction of PB5 to be output and in the next line we set the output of PB5 to high.

On stm32 devices this is more complicated.
The steps are as follows:


1. Find to which Pin the on board LED is connected
2. Turn on the associated port
3. Configure the gpio as output
4. Set the output to high
5. Eventually set the output to low

Before starting download the Reference Manual and the User Manual to your board.

## Find pin
![LEDs](https://github.com/TheSovietStorm/stm32f103rb/blob/master/images/leds.png)
As seen in the User Manual, the User LED we want to control is connected to Arduino Pin D13, which is either PA5 or PB13 depending on the board.
![Pinout](https://github.com/TheSovietStorm/stm32f103rb/blob/master/images/pinout.png)
Scrolling down we see our device and can observe that our board has D13 connected to PA5, which means that to turn on the LED we need to set the fifth pin of Port A high.

## Turn on port
On stm32 we need to access the Reset and Clock Control (RCC) to turn on our GPIO Port.
First of all we need to find where the RCC is located in memory and in the reference manual we can find it swiftly.
![RCC](https://github.com/TheSovietStorm/stm32f103rb/blob/master/images/rccadress.png)
The RCC starts at address 0x40021000 and looking into it we find that the APB2ENR is able to turn on GPIO Port A.
![APB2ENR_1](https://github.com/TheSovietStorm/stm32f103rb/blob/master/images/apb2enr_1.png)
![APB2ENR_2](https://github.com/TheSovietStorm/stm32f103rb/blob/master/images/apb2enr_2.png)

Look for Bit 2 and you see IOPAEN, if you write there a 1, you enable GPIO Port A.
But where is IOPAEN? And where is APB2ENR?
You get the address of the RCC above and in the description of each register you notice an offset value.
If you add that offset to the base address, you receive the address of the register.
In our case the address of APB2ENR will be 0x40021018 because of our base address 0x40021000 and the offset of 0x18.

```
ptr::write(0x4002_1018 as *mut u32, 1<<2);
```

## Configure GPIO as Output

Now that GPIO Port A is enabled, let's look into configuring it.
![PORT](https://github.com/TheSovietStorm/stm32f103rb/blob/master/images/portadress.png)
In the reference manual we find the address of GPIO Port A is 0x40010800 and to configure pins as input/output we need to modify the Port Configure Register Low GPIOX_CRL (if we have a pin above 7 we need GPIOX_CRH).
The first thing we need to do is configure it as an output pin and for our purposes output speed of 2 MHz is sufficient.
The MODEy sets the mode of Pin y, so if we want to modify the Mode of Pin 5, we need to Modify MODE5, which are bits 21 and 20.
We write a 1 to bit 21 for an output mode speed of 2 MHz and the default option in the CNFy is what we want, General Purpose Push Pull and therefore we don't need to modify it.
The offset of GPIOX_CRL is 0x00.
```
ptr::write(0x4001_0800 as *mut u32, 1<<21);
```
Now our GPIO pin is configured and we may set it to high.

## Set the output to high

Looking through the reference manual we learn that the GPIOX_BSRR is used to set specific bits.
We want to write a 1 into Pin 5 and the offset of BSRR is 0x10.
```
ptr::write(0x4001_0810 as *mut u32), 1<<5);
```

Now the LED should be on.

## Eventually set the output to low

Everything needs to have an end, so we look again into the BSRR and find that it has Bits 16 to 31 to reset individual bits.
To clear Pin 5 we need to write into bit (5+16).
```
ptr::write(0x4001_0810 as *mut u32), 1<<(5+16));
```
Now the LED should be off again.


# License

This template is licensed under either of

Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
