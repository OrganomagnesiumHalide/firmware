#include <stdio.h>
#include <stdlib.h>
#include "hardware/uart.h"
#include "pico/stdlib.h"

#include "bridge.h"

void run_rust_main(void);

int main()
{

  stdio_init_all();
  uart_init(uart0, 9600);
  run_rust_main();
}

void c_device_sleep(unsigned int ms)
{
  sleep_ms(ms);
}

void c_gpio_init(unsigned char pin)
{
  gpio_init(pin);
}

void c_gpio_set_dir(unsigned char pin, unsigned char direction)
{
  gpio_set_dir(pin, direction); 
}

void c_gpio_put(unsigned char pin, unsigned char powerLevel)
{
  gpio_put(pin, powerLevel);
}
