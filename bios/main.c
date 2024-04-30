#include "global.h"
#include <string.h>

void bootstrap();
void StartKernel();
void Main(char *config, char *exec);

// bios entrypoint
void reset() {
  // 0xBFC00000
  io.bios_rom = 0x0013243F;
  // 0xBFC00140
  io.common_delay = 0x00031125;
  io.exp1_base_addr = 0x1F000000;
  io.exp2_base_addr = 0x1F802000;
  io.exp1_delay_size = 0x0013243F;
  io.spu_delay = 0x200931e1;
  io.exp3_delay_size = 0x00003022;
  io.exp2_delay_size = 0x00070777;

  // 0xBFC001A0 all registers reset
  asm(
    "OR $at, $zero, $zero\n"
    "OR $v0, $zero, $zero\n"
    "OR $v1, $zero, $zero\n"
    "OR $a0, $zero, $zero\n"
    "OR $a1, $zero, $zero\n"
    "OR $a2, $zero, $zero\n"
    "OR $a3, $zero, $zero\n"
    "OR $t0, $zero, $zero\n"
    "OR $t1, $zero, $zero\n"
    "OR $t2, $zero, $zero\n"
    "OR $t3, $zero, $zero\n"
    "OR $t4, $zero, $zero\n"
    "OR $t5, $zero, $zero\n"
    "OR $t6, $zero, $zero\n"
    "OR $t7, $zero, $zero\n"
    "OR $s0, $zero, $zero\n"
    "OR $s1, $zero, $zero\n"
    "OR $s2, $zero, $zero\n"
    "OR $s3, $zero, $zero\n"
    "OR $s4, $zero, $zero\n"
    "OR $s5, $zero, $zero\n"
    "OR $s6, $zero, $zero\n"
    "OR $s7, $zero, $zero\n"
    "OR $t8, $zero, $zero\n"
    "OR $t9, $zero, $zero\n"
    "OR $k0, $zero, $zero\n"
    "OR $k1, $zero, $zero\n"
    "OR $gp, $zero, $zero\n"
    "OR $sp, $zero, $zero\n"
    "OR $fp, $zero, $zero\n"
    "OR $ra, $zero, $zero\n"
  );

  // 0xBFC0021C icache initialization
  cache_control = 0x00000804;
  cop0_write_status(0x00010000);
  for (int i = 0; i < 0x100; i++) {
    ((uint32_t*)ram_kuseg)[i * 4] = 0;
  }
  cop0_write_status(0x00000000);
  cache_control = 0x00000800;

  // 0xBFC00270 memory initialization
  cop0_write_status(0x00010000);
  for (int i = 0; i < 0x400; i++) {
    ((uint32_t*)ram_kuseg)[i] = 0;
  }
  cop0_write_status(0x00000000);

  // 0xBFC00320
  uint32_t kseg1_read_stub;
  kseg1_read_stub = ((uint32_t*)ram_kseg1)[0];
  kseg1_read_stub = ((uint32_t*)ram_kseg1)[0];
  kseg1_read_stub = ((uint32_t*)ram_kseg1)[0];
  kseg1_read_stub = ((uint32_t*)ram_kseg1)[0];
  kseg1_read_stub = ((uint32_t*)ram_kseg1)[0];
  kseg1_read_stub = ((uint32_t*)ram_kseg1)[0];
  kseg1_read_stub = ((uint32_t*)ram_kseg1)[0];
  kseg1_read_stub = ((uint32_t*)ram_kseg1)[0];

  // 0xBFC00348
  cache_control = 0x0001E988;
  cop0_write_hwrena(0x00000000);
  cop0_write_entrylo1(0x00000000);
  cop0_write_pagemask(0x00000000);
  cop0_write_wired(0x00000000);
  cop0_write_count(0x00000000);
  cop0_write_compare(0x00000000);
  cop0_write_status(0x00000000);
  cop0_write_cause(0x00000000);

  // 0xBFC00398
  for (int i = 0; i < 0xc60; i++) {
    ((uint32_t*)ram_kseg1)[0x2400 + i] = 0;
  }

  // 0xBFC003BC set sp, gp, fp
  asm(
    "li $sp, 0x801FFF00\n"
    "li $gp, 0xA0010FF0\n"
    "move $fp, $sp\n"
  );
  io.mem_size = 0xb80;

  bootstrap();
}

// 0xBFC06E40
void bootstrap() {
  TraceStep(0xF);

  io.spu_reverb_out_r = 0;
  io.spu_reverb_out_l = 0;
  io.spu_main_vol_r = 0;
  io.spu_main_vol_l = 0;

  if (CheckPIO() == 1) {
    ResetPIO();
  }

  // 0xBFC06E80
  TraceStep(0xe);
  ((uint32_t*)ram_kseg1)[0xB980 / 4] = 0;
}

// 0xBFC06734
void StartKernel() {
  char config[80];
  char exec[80];

  strcpy(config, "cdrom0:");
  strcat(config, "SYSTEM.CNF;1");

  strcpy(exec, "cdrom0:");
  strcat(exec, "PSX.EXE;1");

  StartKernel(config, exec);
}

// 0xBFC06798
void Main(char *config, char *exec) {
  TraceStep(1);
}