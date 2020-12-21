pub use self::save_to_amqp::SaveToAmqp;
mod save_to_amqp;
pub use self::save_to_file::SaveToFile;
mod save_to_file;
pub use self::save_to_console::SaveToConsole;
mod save_to_console;