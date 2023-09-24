/// Simplify conditional concatenation of API parameters.
#[macro_export]
macro_rules! add_param {
    ($vec:expr, $val:expr, $name:expr) => {
        if let Some(value) = $val {
            $vec.push(format!("{}={}", $name, value));
        }
    };
}
