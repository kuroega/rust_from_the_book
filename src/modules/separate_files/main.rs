mod sound;

fn main() {
    // absolute path
    crate::sound::instrument::clarinet();

    // relative path
    sound::instrument::clarinet();
}