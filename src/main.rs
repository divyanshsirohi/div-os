// main.rs 
//

#![no_std]  //doesnt link rust standard library
#![no_main]  //disable all Rust level entry points

use core::panic::PanicInfo;

mod vga_buffer;

//dont mangle the name of this function - compiler doesnt change the function name
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    //this function is the entry point since the linker looks for a function named
    //'_start' by default

    println!("Hello World{}", "!");
    panic!("test panic message!");
    loop{}
}


//this function is called upon panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}


 
                                                                                
                    
                                 
                                                                                  
                         
          
 

 

