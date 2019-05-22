mod local_panic{
    #[macro_export]
    macro_rules! local_panic_const_panicking{
        ()=>{
            const E0: usize = [0][3];
        }
    }
}
local_panic_const_panicking!{}

fn main(){}