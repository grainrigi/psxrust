#include "global.h"

// 0xBFC0DE28
static pio_license_text = "Licensed by Sony Computer Entertainment Inc.";

// 0xBFC06FB8
int CheckPIO() {
  char *ptext = pio_license_text;
  char *pio = (char*)&external_pio1[0x21]; // 0x1F000084

  while(*ptext != '\0') {
    if (*ptext != *pio) {
      return;
    }
    ptext++;
    pio++;
  }
  if (*pio != '\0') {
    return 0;
  }
  return 1;
}

// 0xBFC07098
void ResetPIO() {
  // Call 0x1F000080
  ((void (*)(void))&external_pio1[0x20])();
}
