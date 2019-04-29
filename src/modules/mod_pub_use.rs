mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // func body
        }
    }
}

mod performance_group {
    // re-exporting
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn main() {
    performance_group::clarinet_trio();
    // pub use enable main() to access instrument module
    performance_group::instrument::clarinet();
}