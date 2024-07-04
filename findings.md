# Findings

initial tests for setting a pin, still needs to be optimized for speed/size as well:

| Task                      |          | Rust (embassy) | C (ST Cube HAL) |
| :------------------------ | -------- | :------------: | :-------------: |
| Set LED high/low          | t(us)    |      7.25      |       10        |
| Toggle pin                | t(us)    |      8.5       |      11.25      |
| WFI btn and set LED level | t(us)    |       x        |        x        |
|                           | size(kb) |                |                 |

x = scope needed, doing more than one thing with scope at once

both still need some tweaking with optimization flags, but both had some of the basic ones enabled (--release in rust and -Ofast for C).
