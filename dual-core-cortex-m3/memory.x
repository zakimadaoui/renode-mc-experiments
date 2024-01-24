/* Linker script for the STM32F103C8T6 */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 32K
  SHARED : ORIGIN = ORIGIN(RAM) + LENGTH(RAM), LENGTH = 32K
}

SECTIONS 
{
.shared_data ORIGIN(SHARED) :
  {
   *(.shared_data .shared_data.*);
  } > SHARED
}
