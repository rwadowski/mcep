use log::error;

pub fn to_string<T: ToString>(t: T) -> String {
    t.to_string()
}

pub fn log_and_convert_to_string<T: ToString>(t: T) -> String {
    let s = t.to_string();
    error!("{}", s);
    s
}~
