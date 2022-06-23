#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "hardware/uart.h"
#include "pico/stdlib.h"
#include "hardware/i2c.h"
#include "pico/binary_info.h"
#include "lcd-2004/lcd_2004_i2c.h"

#include "bridge.h"

void run_rust_main(void);

int main()
{
  stdio_init_all();
  uart_init(uart0, 9600);
  run_rust_main();
}

void c_print(const void* const s) {
  const char* p = s;
  printf("%s\n",p);
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

void* c_lcd_init(uint8_t i2c_channel, int addr, uint8_t sda_pin, uint8_t scl_pin, int8_t* errorCode) {
    
    struct lcd_device* lcd = malloc(sizeof(struct lcd_device));
    switch (i2c_channel)
    {
    case 0:
      lcd->i2c = i2c0;
      lcd->addr = addr;
      break;
    case 1:
      lcd->i2c = i2c1;
      lcd->addr = addr;
      break;
    default:
      *errorCode = WRONG_I2C_ASSERT_ERROR;
      break;
    }

    i2c_init(lcd->i2c, 100 * 1000);
    gpio_set_function(sda_pin, GPIO_FUNC_I2C);
    gpio_set_function(scl_pin, GPIO_FUNC_I2C);
    gpio_pull_up(sda_pin);
    gpio_pull_up(scl_pin);

    if (!lcd_init(lcd)) {
        *errorCode = FAILED_TO_INIT_LCD_DEVICE;
    }
    *errorCode = 0;
    return lcd;
}

void c_lcd_clear(void* lcd) {
  lcd_clear((struct lcd_device*)lcd);
}
void c_lcd_putch(void* lcd, uint8_t line, int pos, unsigned char ch) {
  lcd_char((struct lcd_device*)lcd, ch);
}