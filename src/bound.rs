use std::ops::{Range, RangeTo, RangeFrom};

pub enum Bound<T> {
	Exact(T),
	Range(T, T),
	RangeFrom(T),
	RangeTo(T)
}

impl<T> From<Range<T>> for Bound<T> {
	fn from(range: Range<T>) -> Self {
		Bound::Range(range.start, range.end)
	}
}

impl<T> From<T> for Bound<T> {
	fn from(value: T) -> Self {
		Bound::Exact(value)
	}
}

impl<T> From<RangeTo<T>> for Bound<T> {
	fn from(range: RangeTo<T>) -> Self {
		Bound::RangeTo(range.end)
	}
}

impl<T> From<RangeFrom<T>> for Bound<T> {
	fn from(range: RangeFrom<T>) -> Self {
		Bound::RangeFrom(range.start)
	}
}