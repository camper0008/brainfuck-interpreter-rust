# brainfuck-interpreter-rust

A brainfuck interpreter written in rust, with inspiration from the [C brainfuck interpreter](https://github.com/camper0008/brainfuck-interpreter-c).
  
Follows the brainfuck specifications, and contrary to the C approach has a "infinite tape" through Vec instead of just allocating ~30000 bytes.
  
When trying to decrement pointer below 0, it panics as this is technically undefined behaviour, and with an infinite tape it's better to force not assuming wrap-around than implementing something that could be buggy if it had an assumed preallocation.
  
When trying to increment pointer above the current allocated memory, simply pushes another byte onto the stack.

Used by executing `<path to binary> <..path to file(s)>`

Can run more than one file simply by adding another argument, though not recommended.

Interprets `bf/bench.bf` in around ~1.615 seconds when compiled for release, according to the standard `time` command, though it's possible to optimize further.
