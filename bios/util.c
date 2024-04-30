#include "global.h"

// 0xBFC01A50
void TraceStep(uint32_t step) {
  jtag_notify = step & 0xFF;
  Bogus1();
}

// 0xBFC03980
static void Bogus1() {
  ((uint32_t*)ram_kseg1)[0x2c1a] = 0;
  ((uint32_t*)ram_kseg1)[0x2c1a] = 0;
  ((uint32_t*)ram_kseg1)[0x2c1a] = 0;
  ((uint32_t*)ram_kseg1)[0x2c1a] = 0;
}
