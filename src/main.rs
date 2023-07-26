//use poliosis_test::run;


mod engine;
mod game;

fn main() {
    loop {
        engine::sound::main().unwrap();
    }
    //pollster::block_on(run());
}