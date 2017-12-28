MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 128K
  RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 32K
}

SECTIONS
{
  /* Set stack top to end of RAM */
  __StackTop = ORIGIN(RAM) + LENGTH(RAM);
  ENTRY(_reset)

  .text : ALIGN(4) 
  {
    /* vector table contains addresses */
    LONG(__StackTop); /* initial stack pointer value */
    KEEP(*(.vector.exceptions)); 
    /* RESET 0x00000004 */
    /* HARD_FAULT @ 0x0000000C */
    /* MEM_FAULT @ 0x00000010 */
    /* BUS_FAULT @ 0x00000014 */
    /* USAGE_FAULT @ 0x00000018 */
    /* SVCall @ 0x0000002C */
    /* PendSV @ 0x00000038 */
    /* SysTick @ 0x0000003C */
    KEEP(*(.vector.interrupts));
 	/* IRQ 0-67 @ 0x00000040 increasing in step of 4 */
    . = 0x150; /* Safe address after IRQ handlers  */

    _TEXT_BEGIN = .;
    *(.text*) /* Program code */
    _TEXT_END = .;
    
    RODATA_BEGIN = .;
    *(.rodata) /* Read only data */
    _RODATA_END = .;
    
    
  } > FLASH
  
  /* "The .data segment contains any global or static variables which
  have a pre-defined value and can be modified. [...]" */
  .data : ALIGN(4)
  {
    *(.data)
  } > RAM
  
  /* "The BSS segment contains all global variables and static variables
  that are initialized to zero or lack explicit initialization [...]" */
  .bss : ALIGN(4)
  {
    _BSS_BEGIN = .;
    *(.bss)
    _BSS_END = .;
  } > RAM

}
