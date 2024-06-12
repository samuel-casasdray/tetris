pub use board_setup::setup_board;
pub use collision::collision_check;
pub use gravity::tetromino_gravity_system;
pub use relative_position::relative_position_system;
pub use tetromino_spawner::tetromino_spawner;

mod board_setup;
mod collision;
mod gravity;
mod relative_position;
mod tetromino_fixer;
mod tetromino_spawner;
mod tetromino_validator;
