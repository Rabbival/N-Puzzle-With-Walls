#[macro_export]
macro_rules! collect_all {
    () => {
        all::<Self>().collect::<Vec<_>>()
    };
}