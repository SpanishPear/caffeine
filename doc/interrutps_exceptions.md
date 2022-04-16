# CPU Interrupts and Exceptions

Generally, there are three types of interrupts.


### Hardware Interrupts
I.e. a keypress, or moving a mouse. Generated externally, either by direct
pin manipulation, or by writing to memory reserved for hardware itnerrupts.

### Software Interrupts
Signalled by software running on CPU to say "oi give me attention plx".
Generally used for kernel syscalls. 0x86, this is the `INT` instruction.

### Exceptions
Alert kernel that something went wrong :(
Double Fault, Page Fault, etc. 

Generally classified into:
  * Faults: something that can be corrected, keep running
  * Traps: report error immediately after execution of instruction that Traps
    * ie, breakpoints :) 
  * Aborts: Something severely went wrong, unrecoverable

On x86 there are about 20 different exception types, most importantly:

  * Page Fault: A page fault occurs on illegal memory accesses. 
  * Invalid Opcode: This exception occurs when the current instruction is invalid, 
      for example when we try to use newer SSE instructions on 
      an old CPU that does not support them.
  * General Protection Fault: This is the exception with the broadest 
      range of causes. 
      It occurs on various kinds of access violations such as trying to 
      execute a privileged instruction in user level code or 
      writing reserved fields in configuration registers.
  * Double Fault: When an exception occurs, the CPU tries to call the 
      corresponding handler function. 
      If another exception occurs while calling the 
      exception handler, the CPU raises a double fault exception. 
      This exception also occurs when there is no handler function registered 
      for an exception.
  * Triple Fault: If an exception occurs while the CPU tries to call 
      the double fault handler function, it issues a fatal triple fault. 
      We can’t catch or handle a triple fault. 
      Most processors react by resetting themselves 
      and/or rebooting the operating system.

Common computer aritechtures keep track of how to handle an interrupt via an 
Interrupt Descriptor Table (IDT).

The IDT is a binary data structure which maps interrupt codes, to an appropriate
function to handle it. On x86, ach entry must have the following 16-byte structure:

Type| Name                     | Description
----|--------------------------|-----------------------------------
u16 | Function Pointer [0:15]  | The lower bits of the pointer to the handler function.
u16 | GDT selector             | Selector of a code segment in the [global descriptor table].
u16 | Options                  | (see below)
u16 | Function Pointer [16:31] | The middle bits of the pointer to the handler function.
u32 | Function Pointer [32:63] | The remaining bits of the pointer to the handler function.
u32 | Reserved                 |

[global descriptor table]: https://en.wikipedia.org/wiki/Global_Descriptor_Table

The options field has the following format:

Bits  | Name                              | Description
------|-----------------------------------|-----------------------------------
0-2   | Interrupt Stack Table Index       | 0: Don't switch stacks, 1-7: Switch to the n-th stack in the Interrupt Stack Table when this handler is called.
3-7   | Reserved              |
8     | 0: Interrupt Gate, 1: Trap Gate   | If this bit is 0, interrupts are disabled when this handler is called.
9-11  | must be one                       |
12    | must be zero                      |
13‑14 | Descriptor Privilege Level (DPL)  | The minimal privilege level required for calling this handler.
15    | Present                           |

Each exception has a predefined IDT index. For example the invalid opcode exception has table index 6 and the page fault exception has table index 14. Thus, the hardware can automatically load the corresponding IDT entry for each exception. The [Exception Table][exceptions] in the OSDev wiki shows the IDT indexes of all exceptions in the “Vector nr.” column.

When an exception occurs, the CPU roughly does the following:

1. Push some registers on the stack, including the instruction pointer and the [RFLAGS] register. (We will use these values later in this post.)
2. Read the corresponding entry from the Interrupt Descriptor Table (IDT). For example, the CPU reads the 14-th entry when a page fault occurs.
3. Check if the entry is present. Raise a double fault if not.
4. Disable hardware interrupts if the entry is an interrupt gate (bit 40 not set).
5. Load the specified [GDT] selector into the CS segment.
6. Jump to the specified handler function.

[RFLAGS]: https://en.wikipedia.org/wiki/FLAGS_register
[GDT]: https://en.wikipedia.org/wiki/Global_Descriptor_Table

Don't worry about steps 4 and 5 for now, we will learn about the global descriptor table and hardware interrupts in future posts.

### Interrupt Descriptor Table

We could make our own, but the x86 crate exposes a 
[InterruptDescriptorTable](https://docs.rs/x86_64/0.14.2/x86_64/structures/idt/struct.InterruptDescriptorTable.html)
struct
