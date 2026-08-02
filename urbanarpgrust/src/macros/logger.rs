#[macro_export]
macro_rules! log_info{
    ($($arg:tt)*) => {
        godot::prelude::godot_print!("[{}:{}] {}",
            module_path!(),
            line!(),
            format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! log_error{
    ($($arg:tt)*) => {
        godot::prelude::godot_error!("[{}:{}] {}",
            module_path!(),
            line!(),
            format!($($arg)*)
        )
    };
}