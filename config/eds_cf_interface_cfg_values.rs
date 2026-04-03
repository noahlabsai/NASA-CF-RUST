#[macro_export]
macro_rules! CF_INTERFACE_CFGVAL {
    ($x:ident) => {
        concat!("EdsParam_CF_", stringify!($x))
    };
}