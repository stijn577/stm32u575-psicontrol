# Findings

initial tests for setting a pin, still needs to be optimized for speed/size as well:

| Task             |                  | Rust opt for size  | C opt for size | Rust opt for speed | C opt for speed |
| :--------------- | ---------------- | ------------------ | -------------- | ------------------ | --------------- |
| Set pin high/low | freq (xHz)       | X                  | X              | X                  | X               |
|                  | .text/.data/.bss | 11808/8/33108      | 6336/12/1572   | 13376/8/33100      | 7048/12/1572    |
| Set pin raw reg  | freq (xHz)       | link err (for now) | X              | link err (for now) | X               |
|                  |                  | link err (for now) | 6320/12/1572   | link err (for now) | 7032/12/1572    |
| polling button   | ...              | X                  | X              | X                  | X               |
|                  |                  | 11864/8/33096      | 6372/12/1572   | 13432/8/33100      | 7096/12/1572    |
| exti button      | delay (xs)       | X                  | X              | X                  | X               |
|                  |                  | 12560/8/33096      | 7880/12/1644   | 14340/8/33100      | 8704/12/1644    |

X = scope needed, doing more than one thing with scope at once, so waiting for now

both were compiled with plenty of flags, the barebon amount of peripherals were used and the system clock was running at max speed (160MHz). Notice the Rust code is quite large in the .bss section. The reason for this is because embassy is a no_alloc framework. In order to avoid needing heap allocations for the tasks the tasks are stack allocated at compile time (and initialized later). The rust .text section still seems to do double the size for these small examples. This can be attributed to the fact that we are using embassy as well
