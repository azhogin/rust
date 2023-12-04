// ignore-lldb
#![feature(collapse_debuginfo)]

// Test that statement, skipped by macros, is correctly processed in debuginfo

// compile-flags:-g

// === GDB TESTS ===================================================================================

// gdb-command:run
// gdb-command:next
// gdb-command:frame
// gdb-check:[...]#loc2[...]
// gdb-command:step
// gdb-command:frame
// gdb-check:[...]#loc_call1[...]
// gdb-command:finish
// gdb-check:[...]#loc4[...]
// gdb-command:next
// gdb-command:frame
// gdb-check:[...]#loc5[...]
// gdb-command:continue


// Macro accepts 3 statements and removes the 2nd statement
macro_rules! remove_second_statement {
    ($s1:stmt; $s2:stmt; $s3:stmt;) => { $s1 $s3 }
}

fn call1() {
    println!("one"); // #loc_call1
}

fn call2() {
    println!("two"); // #loc_call2
}

fn call3() {
    println!("three"); // #loc_call3
}

fn main() {
    let ret = 0; // #break, step should go to call1
    remove_second_statement! { // #loc1
        call1(); // #loc2, breakpoint should set to call1, step should go call3
        call2(); // #loc3, breakpoint should set to call3
        call3(); // #loc4, breakpoint should set to call3, step should go closing brace
    }
    std::process::exit(ret); // #loc5
}

