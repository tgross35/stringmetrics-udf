use libc;


enum ItemResult {
	InvalidResult,
	StringResult,
	RealResult,
	IntResult,
	RowResult,
	DecimalResult
}


#[repr(C)]
struct UDF_ARGS {
	pub arg_count: libc::c_uint,

}

impl UDF_ARGS {
	fn to_safe(&self) -> UdfArgs {
		:w
	}

}

// maybe want to load/dump this struct to a UDF_ARGS pointer
struct UdfArgs {
	
}


