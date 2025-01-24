use color_print::cformat;

#[macro_export]
macro_rules! error {
    ($($token:tt)+) => { 
        #[cfg(not(feature = "wasm"))] {
            let head = cformat!("<r>error:</>");
            print!("{head}");
            println!($($token),*);
        }
    }
}

#[macro_export]
macro_rules! warning {
    ($($token:tt)+) => { 
        #[cfg(not(feature = "wasm"))] {
            let head = cformat!("<y>warning:</>");
            print!("{head}");
            println!($($token),*);
        }
    }
}



