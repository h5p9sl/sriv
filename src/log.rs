#[macro_export]
macro_rules! log_verbose {
    () => (std::eprint!("\n"));
    ($($args:tt)*) => ({
        use colored::*;
        eprintln!("{}: {}", "[INFO]".yellow(), format_args!($($args)*));
    })
}

#[macro_export]
macro_rules! benchmark {

    ($($st:stmt)*) => {
        let mut _start = std::time::Instant::now();
        let mut offs = 0;
        $(
            offs += 1;
            $st
            let elapsed = _start.elapsed().as_millis();
            _start = std::time::Instant::now();
            log_verbose!("{}:{}+{}: {}ms -> \"{:}\"", file!(), line!(), offs, elapsed, stringify!($st));
         )*
    };

    ($fn:expr) => ({
        let start = std::time::Instant::now();
        let x = $fn;
        let elapsed = start.elapsed().as_secs_f32();
        log_verbose!("{}: +{}", stringify!($fn), elapsed);
        x
    });
}
