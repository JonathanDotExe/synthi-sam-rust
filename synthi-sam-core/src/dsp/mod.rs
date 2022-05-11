pub mod oscillator;

#[inline]
pub fn note_to_freq_transpose (note: f64) -> f64 {
	return f64::from(2.0).powf(note/12.0);
}

#[inline]
pub fn note_to_freq (note: f64) -> f64 {
    return 440.0 * f64::from(2.0).powf((note - 69.0)/12.0);
}

