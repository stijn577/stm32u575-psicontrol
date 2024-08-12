# A command to disassemble a .elf file to assembly code
def rasm [
    path: path = ./target/thumbv8m.main-none-eabihf/release/app, # path to the .elf file that needs to be disassembled
    --save (-s), # save the file with ascii art (no colors) to a file
] {
    if not ($save) {
        arm-none-eabi-objdump -m arm --visualize-jumps=color --disassembler-color=on --demangle="rust" --inlines --wide --source --demangle -d $path
    } else {
        if not ($path | path exists) {
            error make -u {msg: "path does not exist", }
        }

        # create asm directory if it doesn't exist, so save doesn't error 
        if not ("asm" | path exists) {
            mkdir asm
        }

        # dump the output from elf to an asm file
        arm-none-eabi-objdump -m arm --visualize-jumps --demangle="rust" --inlines --wide --source --demangle -d $path | save (["./asm/rust/asm_", ($path | path basename), ".out" ] | str join) -f
    } 
}


# A command to disassemble a .elf file to assembly code
def casm [
    path: path # path to the .elf file that needs to be disassembled
    --save (-s), # save the file with ascii art (no colors) to a file
] {
    if not ($save) {
        arm-none-eabi-objdump -m arm --visualize-jumps=color --disassembler-color=on --demangle="gnu-v3" --inlines --wide --source --demangle -d $path
    } else {
        if not ($path | path exists) {
            error make -u {msg: "path does not exist", }
        }

        # create asm directory if it doesn't exist, so save doesn't error 
        if not ("asm" | path exists) {
            mkdir asm
        }

        # dump the output from elf to an asm file
        arm-none-eabi-objdump -m arm --visualize-jumps --demangle="gnu-v3" --inlines --wide --source --demangle -d $path | save (["./asm/c/asm_", ($path | path basename), ".out" ] | str join) -f
    } 
}
