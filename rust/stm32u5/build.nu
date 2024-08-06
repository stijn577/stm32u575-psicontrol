# A command to disassemble a .elf file to assembly code
def disassemble [
    path: path = ./target/thumbv8m.main-none-eabihf/release/app, # path to the .elf file that needs to be disassembled
    --save (-s), # save the file with ascii art (no colors) to a file
    --bat # if this is enabled the output is piped into bat (try not to use --save simultaneously, it removes colors in the output making it harder to read)
] {
    if not ($save) {
        let out = arm-none-eabi-objdump -m arm --visualize-jumps=color --disassembler-color=on --demangle="rust" --inlines --wide --source --demangle -d $path
        if ($bat) {
            bat $out
        } else {
            $out
        }
    } else {
        if not ($path | path exists) {
            error make -u {msg: "path does not exist", }
        }

        # create asm directory if it doesn't exist, so save doesn't error 
        if not ("asm" | path exists) {
            mkdir asm
        }

        # dump the output from elf to an asm file
        arm-none-eabi-objdump -m arm --visualize-jumps --demangle="rust" --inlines --wide --source --demangle -d $path | save (["./asm/asm_", ($path | path basename), ".out" ] | str join) -f
    } 
}
