#include <stdint.h>

struct io_regs {
  // 0x1F801000
  uint32_t exp1_base_addr;
  // 0x1F801004
  uint32_t exp2_base_addr;
  // 0x1F801008
  uint32_t exp1_delay_size;
  // 0x1F80100C
  uint32_t exp3_delay_size;
  // 0x1F801010
  uint32_t bios_rom;
  // 0x1F801014
  uint32_t spu_delay;
  // 0x1F801018
  uint32_t cdrom_delay;
  // 0x1F80101C
  uint32_t exp2_delay_size;
  // 0x1F801020
  uint32_t common_delay;
  // 0x1F801024 - 0x1F80103F
  uint32_t pad1[0x7];
  // 0x1F801040
  uint32_t joy_mcd_data;
  // 0x1F801044
  uint32_t joy_mcd_stat;
  // 0x1F801048
  uint16_t joy_mcd_mode;
  // 0x1F80104A
  uint32_t joy_mcd_control;
  // 0x1F80104E
  uint16_t joy_mcd_baud;
  // 0x1F801050
  uint32_t sio_data;
  // 0x1F801054
  uint32_t sio_stat;
  // 0x1F801058
  uint16_t sio_mode;
  // 0x1F80105A
  uint16_t sio_control;
  // 0x1F80105C
  uint16_t sio_misc;
  // 0x1F80105E
  uint16_t sio_baud;
  // 0x1F801060
  uint32_t mem_size;
  // 0x1F801064 - 0x1F801D7F
  uint32_t pad2[0x1B9];
  // 0x1F801D80
  uint16_t spu_main_vol_l;
  // 0x1F801D82
  uint16_t spu_main_vol_r;
  // 0x1F801D84
  uint16_t spu_reverb_out_l;
  // 0x1F801D86
  uint16_t spu_reverb_out_r;
};

// 0x00000000
void *ram_kuseg;
// 0xA0000000
void *ram_kseg1;

// start at 0x1F000000
volatile uint32_t external_pio1[8192 * 1024 / 4];
// start at 0x1F801000
volatile struct io_regs io;
// 0x1F802041
volatile uint32_t jtag_notify;

// 0xFFFE0130
volatile uint32_t cache_control;

uint32_t cop0_read_status();
void cop0_write_status(uint32_t value);
void cop0_write_hwrena(uint32_t value);
void cop0_write_entrylo1(uint32_t value);
void cop0_write_pagemask(uint32_t value);
void cop0_write_wired(uint32_t value);
void cop0_write_count(uint32_t value);
void cop0_write_compare(uint32_t value);
void cop0_write_cause(uint32_t value);

void TraceStep(uint32_t step);
int CheckPIO();
void ResetPIO();