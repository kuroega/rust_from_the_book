mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
            super::breathe_in();
        }
    }

    fn breathe_in() {
    	// Function body
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}


// module tree
// -----------
// crate
// └── sound
//     ├── instrument
//     │   
//     └── voice