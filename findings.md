# Findings

initial tests for setting a pin:

| Task             | Rust (embassy) | C (ST Cube HAL) |
| :--------------- | :------------: | :-------------: |
| Set LED high/low |      7.25      |       10        |
| Toggle pin       |      8.5       |      11.25      |

both still need some tweaking with optimization flags, but both had some of the basic ones enabled.
