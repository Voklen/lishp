#[macro_export]
macro_rules! unwrap_or_return {
	( $e:expr ) => {
		match $e {
			Some(x) => x,
			None => return,
		}
	};
}

#[macro_export]
macro_rules! throw{
    ($($message:tt)*) => {{
		use	crate::errors::throw_error_fuction;
        let res = format!($($message)*);
        throw_error_fuction(res)
    }}
}

pub fn throw_error_fuction(error_message: String) -> ! {
	#[cfg(not(debug_assertions))]
	exit_production(error_message);
	#[cfg(debug_assertions)]
	panic!("{error_message}");
}

#[allow(dead_code)]
fn exit_production(error_message: String) -> ! {
	let program_name = env!("CARGO_PKG_NAME");
	eprintln!("{program_name}: {error_message}");
	std::process::exit(1);
}
