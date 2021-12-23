# brainfuck-interpreter-rust

A brainfuck interpreter rewritten in rust, originally written in C.
  
Follows the brainfuck specifications, and contrary to the C approach has a "infinite tape" through Vec instead of just allocating ~30000 bytes.
  
When trying to decrement pointer below 0, it panics as this is technically undefined behaviour, and with an infinite tape it's better to force not assuming wrap-around than implementing something that could be buggy if it had an assumed preallocation.
  
When trying to increment pointer above the current allocated memory, simply pushes another byte onto the stack.
