main:
addi x29, x0, 5   // Add 5 and 0, and store the value to x29.
addi x30, x0, 37  // Add 37 and 0, and store the value to x30.
sub x31, x30, x29 // Subtract x29 from x30, and store the value to x31 (should be 0x20).