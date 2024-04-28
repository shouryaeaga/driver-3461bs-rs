## Platform Agnostic Rust Driver for the 3461BS1 4 Digit 7 Segment Display
A super simple rust driver for any 4 digit 7 segment display, currently only tested on the 3461BS1, however, theoretically it should work for any. If you find any errors with different displays or boards, please let me know.
## Example
A working example with the Raspberry Pi Pico.

Here is the pinout for the 3461BS1:
![](https://www.zpag.net/Electroniques/Arduino/images/img164.jpg)

```rust
let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

// Setup for all the required pins
let  mut  e  =  pins.gpio1.into_push_pull_output();
let  mut  d  =  pins.gpio2.into_push_pull_output();
let  mut  dp  =  pins.gpio3.into_push_pull_output();
let  mut  c  =  pins.gpio4.into_push_pull_output();
let  mut  g  =  pins.gpio5.into_push_pull_output();
let  mut  d4  =  pins.gpio6.into_push_pull_output();
let  mut  d1  =  pins.gpio7.into_push_pull_output();
let  mut  a  =  pins.gpio8.into_push_pull_output();
let  mut  f  =  pins.gpio9.into_push_pull_output();
let  mut  d2  =  pins.gpio10.into_push_pull_output();
let  mut  d3  =  pins.gpio11.into_push_pull_output();
let  mut  b  =  pins.gpio12.into_push_pull_output();

// Creating the LED3461BS
let  mut  led_display  =  LED3461BS1::new(
&mut  a, &mut  b, &mut  c, &mut  d, &mut  e, &mut  f, &mut  g, &mut  dp, &mut  d1, &mut  d2,
&mut  d3, &mut  d4,
);

// Displaying characters 2 3 4 C on the board
// Delay 5 milliseconds before settings the next digit, due to the optical illusion of [persistance of vision](https://en.wikipedia.org/wiki/Persistence_of_vision)
loop {
	// To set a number, first parameter is a char with the digit you want it to be on
	// Second parameter is a char with the number on the digit
	led_display.set_number_on_digit('1', '2').unwrap();
	delay.delay_ms(5);
	led_display.clear();
	led_display.set_number_on_digit('2', '3').unwrap();
	delay.delay_ms(5);
	led_display.clear();
	led_display.set_number_on_digit('2', '4').unwrap();
	delay.delay_ms(5);
	led_display.clear();
	led_display.set_number_on_digit('2', '5').unwrap();
	delay.delay_ms(5);
	led_display.clear();
	// To set a custom character, first parameter should be the digit you want it to be on
	// The next parameters are segments a-g
	// The last parameter is the decimal point
	// Look at the pinout of the 3461BS1
	led_display
	.set_custom_character('4', true, false, false, true, true, true, false, false)
	.unwrap();
}
 ```
