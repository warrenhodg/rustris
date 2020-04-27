pub mod block;

pub use self::block::Block;

pub mod colour;
pub use self::colour::Colour;

pub mod game;
pub use self::game::Game;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
