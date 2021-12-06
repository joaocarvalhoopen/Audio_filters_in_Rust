/// Project: Audio filters in Rust
/// Date:    2021.12.05
/// Author of the port: João Nuno Carvalho
///
/// Description: Audio or DSP filters, allow you to attenuate or accentuate some frequencies
///              or range of frequencies in a signal. The signal can be of any kind, but in
///              here, we will focus on 1D signals. Like audio signals.
///              There can also occur differences in the signal phases, that vary with the
///              filter and the frequency components of the signal.  
///              This is a port of Audio filters, from Python to Rust,
///              from the Audio filter from TheAlgorithms GitHub in Python. That is by it
///              self a port from WebAudio API implementation of the same common
///              filters in the browsers.
/// 
///              The following filters are implemented over a BiQuad IIR filter:
///                 -lowpass
///                 -highpass
///                 -bandpass
///                 -allpass
///                 -peak
///                 -lowshelf
///                 -highshelf 
///  
/// License: MIT Open Source License, like the original license from
///    GitHub - TheAlgorithms / Python / audio_filters
///    https://github.com/TheAlgorithms/Python/tree/master/audio_filters
///
/// How to run the code. 
/// 
/// To make a project for this files do:
///     -Install Rust your computer (Linux, Win, Mac, Raspberry Pi).
///     
///     cargo new audio_filters_in_rust
///     cd audio_filters_in_rust
///     
///     -Copy the repository files to this directory and overlap them.
/// 
/// To compile do:
///     cargo build --release
/// 
/// To run do:
///     cargo run --release
/// 
/// to run the tests do:
///     cargo test
/// 
/// References:
///    1. GitHub - TheAlgorithms / Python / audio_filters
///       https://github.com/TheAlgorithms/Python/tree/master/audio_filters
///
///    2. Good resources on DSP – Digital Signal Programming
///       https://github.com/joaocarvalhoopen/How_to_learn_modern_electronics#dsp--digital-signal-programming
///
///    3. WebAudio API - Mozilla Docs
///       https://developer.mozilla.org/pt-BR/docs/Web/API/Web_Audio_API
/// 
///    4. Audio Filters - Theory and Practice
///       by Ethan Winer
///       http://ethanwiner.com/filters.html
/// 
///    5. Audio filter - Wikipedia
///       https://en.wikipedia.org/wiki/Audio_filter
/// 
///    6. Electronic filter - Wikipedia
///       https://en.wikipedia.org/wiki/Electronic_filter
///
///    7. How to learn modern Rust
///       https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust
/// 


use crate::iir_filter::IIRFilter;
use std::f64::consts::TAU;

/// Create 2nd-order IIR filters with Butterworth design.
/// 
///  Code based on https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html
///  Alternatively you can use scipy.signal.butter, which should yield the same results.
/// 


/// Creates a low-pass filter
///
/// In Python: 
///    >>> filter = make_lowpass(1000, 48000)
///    >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
///    [1.0922959556412573, -1.9828897227476208, 0.9077040443587427, 0.004277569313094809,
///    0.008555138626189618, 0.004277569313094809]
/// 
/// In Rust:
///    >>> let filter = make_lowpass(1000, 48000);
///    >>> let res_coeffs: Vec<f64> = filter.a_coeffs.iter.extends(filter.b_coeffs).collect();
///    >>> println!("{}", res_coeffs);
///    [1.0922959556412573, -1.9828897227476208, 0.9077040443587427, 0.004277569313094809,
///    0.008555138626189618, 0.004277569313094809]
///
pub fn make_lowpass(frequency: f64, sample_rate: u32, q_factor: Option<f64>) -> IIRFilter {
    let q_factor: f64 = if q_factor.is_none() {
                                1.0 / f64::sqrt(2.0)
                        } else {
                            q_factor.unwrap()
                        };

        let w0 = TAU * frequency / sample_rate as f64;
        let _sin = f64::sin(w0);
        let _cos = f64::cos(w0);
        let alpha = _sin / (2.0 * q_factor);
    
        let b0 = (1.0 - _cos) / 2.0;
        let b1 = 1.0 - _cos;
    
        let a0 =  1.0 + alpha;
        let a1 = -2.0 * _cos;
        let a2 =  1.0 - alpha;
    
        let filter_order = 2;
        let mut filter = IIRFilter::new(filter_order);
        let _ = filter.set_coefficients(& [a0, a1, a2], & [b0, b1, b0]);
        
        filter
}

/// Creates a high-pass filter
/// 
/// In Python:
///    >>> filter = make_highpass(1000, 48000)
///    >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
///    [1.0922959556412573, -1.9828897227476208, 0.9077040443587427, 0.9957224306869052,
///    -1.9914448613738105, 0.9957224306869052]
/// 
pub fn make_highpass(frequency: f64, sample_rate: u32, q_factor: Option<f64>) -> IIRFilter {
    let q_factor: f64 = if q_factor.is_none() {
                                1.0 / f64::sqrt(2.0)
                        } else {
                            q_factor.unwrap()
                        };

    let w0 = TAU * frequency / sample_rate as f64; 
    let _sin = f64::sin(w0);
    let _cos = f64::cos(w0);
    let alpha = _sin / (2.0 * q_factor);

    let b0 = (1.0 + _cos) / 2.0;
    let b1 = -1.0 - _cos;

    let a0 =  1.0 + alpha;
    let a1 = -2.0 * _cos;
    let a2 =  1.0 - alpha;

    let filter_order = 2;
    let mut filter = IIRFilter::new(filter_order);
    let _ = filter.set_coefficients(& [a0, a1, a2], & [b0, b1, b0]);
    
    filter
}

/// Creates a band-pass filter
/// 
/// In Python:
///     >>> filter = make_bandpass(1000, 48000)
///     >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
///     [1.0922959556412573, -1.9828897227476208, 0.9077040443587427, 0.06526309611002579,
///     0, -0.06526309611002579]
/// 
pub fn make_bandpass(frequency: f64, sample_rate: u32, q_factor: Option<f64>) -> IIRFilter {
    let q_factor: f64 = if q_factor.is_none() {
                                1.0 / f64::sqrt(2.0)
                        } else {
                            q_factor.unwrap()
                        };

    let w0 = TAU * frequency / sample_rate as f64;
    let _sin = f64::sin(w0);
    let _cos = f64::cos(w0);
    let alpha = _sin / (2.0 * q_factor);

    let b0 = _sin / 2.0;
    let b1 = 0.0;
    let b2 = -b0;

    let a0 =  1.0 + alpha;
    let a1 = -2.0 * _cos;
    let a2 =  1.0 - alpha;

    let filter_order = 2;
    let mut filter = IIRFilter::new(filter_order);
    let _ = filter.set_coefficients(& [a0, a1, a2], & [b0, b1, b2]);
    
    filter
}

/// Creates an all-pass filter
/// 
/// In Python:
///     >>> filter = make_allpass(1000, 48000)
///     >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
///     [1.0922959556412573, -1.9828897227476208, 0.9077040443587427, 0.9077040443587427,
///     -1.9828897227476208, 1.0922959556412573]
///
pub fn make_allpass(frequency: f64, sample_rate: u32, q_factor: Option<f64>) -> IIRFilter {
    let q_factor: f64 = if q_factor.is_none() {
                                1.0 / f64::sqrt(2.0)
                        } else {
                            q_factor.unwrap()
                        };

    let w0 = TAU * frequency / sample_rate as f64;
    let _sin = f64::sin(w0);
    let _cos = f64::cos(w0);
    let alpha = _sin / (2.0 * q_factor);

    let b0 =  1.0 - alpha;
    let b1 = -2.0 * _cos;
    let b2 =  1.0 + alpha;

    let filter_order = 2;
    let mut filter = IIRFilter::new(filter_order);
    let _ = filter.set_coefficients(& [b2, b1, b0], & [b0, b1, b2]);
    
    filter
}

/// Creates a peak filter
///
/// In Python: 
///     >>> filter = make_peak(1000, 48000, 6)
///     >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
///     [1.0653405327119334, -1.9828897227476208, 0.9346594672880666, 1.1303715025601122,
///     -1.9828897227476208, 0.8696284974398878]
///
pub fn make_peak(frequency: f64, sample_rate: u32, gain_db: f64, q_factor: Option<f64>) -> IIRFilter {
    let q_factor: f64 = if q_factor.is_none() {
                                1.0 / f64::sqrt(2.0)
                        } else {
                            q_factor.unwrap()
                        };

    let w0 = TAU * frequency / sample_rate as f64;
    let _sin = f64::sin(w0);
    let _cos = f64::cos(w0);
    let alpha = _sin / (2.0 * q_factor);
    let big_a = 10.0_f64.powf(gain_db / 40.0);

    let b0 =  1.0 + alpha * big_a;
    let b1 = -2.0 * _cos;
    let b2 =  1.0 - alpha * big_a;
    let a0 =  1.0 + alpha / big_a;
    let a1 = -2.0 * _cos;
    let a2 =  1.0 - alpha / big_a;

    let filter_order = 2;
    let mut filter = IIRFilter::new(filter_order);
    let _ = filter.set_coefficients(& [a0, a1, a2], & [b0, b1, b2]);
    
    filter

}

/// Creates a low-shelf filter
/// 
/// In Python:
///     >>> filter = make_lowshelf(1000, 48000, 6)
///     >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
///     [3.0409336710888786, -5.608870992220748, 2.602157875636628, 3.139954022810743,
///      -5.591841778072785, 2.5201667380627257]
/// 
pub fn make_lowshelf(frequency: f64, sample_rate: u32, gain_db: f64, q_factor: Option<f64>) -> IIRFilter {
    let q_factor: f64 = if q_factor.is_none() {
                                1.0 / f64::sqrt(2.0)
                        } else {
                            q_factor.unwrap()
                        };

    let w0 = TAU * frequency / sample_rate as f64;
    let _sin = f64::sin(w0);
    let _cos = f64::cos(w0);
    let alpha = _sin / (2.0 * q_factor);
    let big_a = 10.0_f64.powf(gain_db / 40.0);
    let pmc = (big_a + 1.0) - (big_a - 1.0) * _cos;
    let ppmc = (big_a + 1.0) + (big_a - 1.0) * _cos;
    let mpc = (big_a - 1.0) - (big_a + 1.0) * _cos;
    let pmpc = (big_a - 1.0) + (big_a + 1.0) * _cos;
    let aa2 = 2.0 * f64::sqrt(big_a) * alpha;

    let b0 = big_a * (pmc + aa2);
    let b1 = 2.0 * big_a * mpc;
    let b2 = big_a * (pmc - aa2);
    let a0 = ppmc + aa2;
    let a1 = -2.0 * pmpc;
    let a2 = ppmc - aa2;

    let filter_order = 2;
    let  mut filter = IIRFilter::new(filter_order);
    let _ = filter.set_coefficients(& [a0, a1, a2], & [b0, b1, b2]);
    
    filter
}

/// Creates a high-shelf filter
///
/// In Python: 
///     >>> filter = make_highshelf(1000, 48000, 6)
///     >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
///     [2.2229172136088806, -3.9587208137297303, 1.7841414181566304, 4.295432981120543,
///      -7.922740859457287, 3.6756456963725253]
///
pub fn make_highshelf(frequency: f64, sample_rate: u32, gain_db: f64, q_factor: Option<f64>) -> IIRFilter {
    let q_factor: f64 = if q_factor.is_none() {
                                1.0 / f64::sqrt(2.0)
                        } else {
                            q_factor.unwrap()
                        };

    let w0 = TAU * frequency / sample_rate as f64;
    let _sin = f64::sin(w0);
    let _cos = f64::cos(w0);
    let alpha = _sin / (2.0 * q_factor);
    let big_a = 10.0_f64.powf(gain_db / 40.0);
    let pmc = (big_a + 1.0) - (big_a - 1.0) * _cos;
    let ppmc = (big_a + 1.0) + (big_a - 1.0) * _cos;
    let mpc = (big_a - 1.0) - (big_a + 1.0) * _cos;
    let pmpc = (big_a - 1.0) + (big_a + 1.0) * _cos;
    let aa2 = 2.0 * f64::sqrt(big_a) * alpha;

    let b0 = big_a * (ppmc + aa2);
    let b1 = -2.0 * big_a * pmpc;
    let b2 = big_a * (ppmc - aa2);
    let a0 = pmc + aa2;
    let a1 = 2.0 * mpc;
    let a2 = pmc - aa2;

    let filter_order = 2;
    let mut filter = IIRFilter::new(filter_order);
    let _ = filter.set_coefficients(& [a0, a1, a2], & [b0, b1, b2]);
    
    filter
}


#[cfg(test)]
mod tests {
    use super::*;

    fn print_values(target_vec: & Vec<f64>, res_coeffs: & Vec<&f64>) {
        println!("\n >>>> target_coefficents");
        for str_t in target_vec {
            print!("{}, ", str_t);    
        }
        println!("\n >>>> res_coefficents");
        for str_t in res_coeffs {
            print!("{}, ", str_t);    
        }
        println!("");
    }

    #[test]
    fn test_make_lowpass() {
        // >>> filter = make_lowpass(1000, 48000)
        // >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
        // [1.0922959556412573, -1.9828897227476208, 0.9077040443587427, 0.004277569313094809,
        // 0.008555138626189618, 0.004277569313094809]
       
        let frequency = 1_000.0;  // Hz
        let sample_rate = 48_000; // Samples
        let filter = make_lowpass(frequency, sample_rate, None);
        
        let target_vec = vec![1.0922959556412573, -1.9828897227476208, 0.9077040443587427,
                                      0.004277569313094809, 0.008555138626189618, 0.004277569313094809];
        
        let res_coeffs: Vec<&f64> = filter.a_coeffs.iter().chain(filter.b_coeffs.iter()).collect();
        print_values(& target_vec, & res_coeffs);        
        for i in 0..target_vec.len() {
            assert_eq!(*(res_coeffs[i]), target_vec[i]);
        }

        // assert_eq!(true, false);
    }

    #[test]
    fn test_make_highpass() {
        // >>> filter = make_highpass(1000, 48000)
        // >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
        // [1.0922959556412573, -1.9828897227476208, 0.9077040443587427, 0.9957224306869052,
        // -1.9914448613738105, 0.9957224306869052]
       
        let frequency = 1_000.0;  // Hz
        let sample_rate = 48_000; // Samples
        let filter = make_highpass(frequency, sample_rate, None);

        let target_vec = vec![1.0922959556412573, -1.9828897227476208, 0.9077040443587427,
                                      0.9957224306869052, -1.9914448613738105, 0.9957224306869052];
        
        let res_coeffs: Vec<&f64> = filter.a_coeffs.iter().chain(filter.b_coeffs.iter()).collect();
        print_values(& target_vec, & res_coeffs);
        for i in 0..target_vec.len() {
            assert_eq!(*(res_coeffs[i]), target_vec[i]);
        }

        // assert_eq!(true, false);
    }

    #[test]
    fn test_make_bandpass() {
        //     >>> filter = make_bandpass(1000, 48000)
        //     >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
        //     [1.0922959556412573, -1.9828897227476208, 0.9077040443587427, 0.06526309611002579,
        //     0, -0.06526309611002579]
       
        let frequency = 1_000.0;  // Hz
        let sample_rate = 48_000; // Samples
        let filter = make_bandpass(frequency, sample_rate, None);

        let target_vec = vec![1.0922959556412573, -1.9828897227476208, 0.9077040443587427,
                                      0.06526309611002579, 0.0, -0.06526309611002579];

        let res_coeffs: Vec<&f64> = filter.a_coeffs.iter().chain(filter.b_coeffs.iter()).collect();
        print_values(& target_vec, & res_coeffs);
        for i in 0..target_vec.len() {
            assert_eq!(*(res_coeffs[i]), target_vec[i]);
        }

        // assert_eq!(true, false);
    }

    #[test]
    fn test_make_allpass() {
        // >>> filter = make_allpass(1000, 48000)
        // >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
        // [1.0922959556412573, -1.9828897227476208, 0.9077040443587427, 0.9077040443587427,
        // -1.9828897227476208, 1.0922959556412573]
        
        let frequency = 1_000.0;  // Hz
        let sample_rate = 48_000; // Samples
        let filter = make_allpass(frequency, sample_rate, None);
        
        let target_vec = vec![1.0922959556412573, -1.9828897227476208, 0.9077040443587427,
                                       0.9077040443587427, -1.9828897227476208, 1.0922959556412573];
        
        let res_coeffs: Vec<&f64> = filter.a_coeffs.iter().chain(filter.b_coeffs.iter()).collect();
        print_values(& target_vec, & res_coeffs);
        for i in 0..target_vec.len() {
            assert_eq!(*(res_coeffs[i]), target_vec[i]);
        }

        // assert_eq!(true, false);
    }

    #[test]
    fn test_make_peak() {
        // >>> filter = make_peak(1000, 48000, 6)
        // >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
        // [1.0653405327119334, -1.9828897227476208, 0.9346594672880666, 1.1303715025601122,
        // -1.9828897227476208, 0.8696284974398878]

        let frequency = 1_000.0;  // Hz
        let sample_rate = 48_000; // Samples
        let gain_db = 6.0;        // dB
        let filter = make_peak(frequency, sample_rate, gain_db, None);
        
        let target_vec = vec![1.0653405327119334, -1.9828897227476208, 0.9346594672880666,
                                      1.1303715025601122, -1.9828897227476208, 0.8696284974398878];
        
        let res_coeffs: Vec<&f64> = filter.a_coeffs.iter().chain(filter.b_coeffs.iter()).collect();
        print_values(& target_vec, & res_coeffs);
        for i in 0..target_vec.len() {
            assert_eq!(*(res_coeffs[i]), target_vec[i]);
        }

        // assert_eq!(true, false);
    }

    #[test]
    fn test_make_lowshelf() {
        // >>> filter = make_lowshelf(1000, 48000, 6)
        // >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
        // [3.0409336710888786, -5.608870992220748, 2.602157875636628, 3.139954022810743,
        // -5.591841778072785, 2.5201667380627257]

        let frequency = 1_000.0;  // Hz
        let sample_rate = 48_000; // Samples
        let gain_db = 6.0;        // dB
        let filter = make_lowshelf(frequency, sample_rate, gain_db, None);

        let target_vec = vec![3.0409336710888786, -5.608870992220748, 2.602157875636628,
                                      3.139954022810743, -5.591841778072785, 2.5201667380627257];

        let res_coeffs: Vec<&f64> = filter.a_coeffs.iter().chain(filter.b_coeffs.iter()).collect();
        print_values(& target_vec, & res_coeffs);
        for i in 0..target_vec.len() {
            assert_eq!(*(res_coeffs[i]), target_vec[i]);
        }

        // assert_eq!(true, false);
    }

    #[test]
    fn test_make_highshelf() {
        // >>> filter = make_highshelf(1000, 48000, 6)
        // >>> filter.a_coeffs + filter.b_coeffs  # doctest: +NORMALIZE_WHITESPACE
        // [2.2229172136088806, -3.9587208137297303, 1.7841414181566304, 4.295432981120543,
        // -7.922740859457287, 3.6756456963725253]
        //
        let frequency = 1_000.0;  // Hz
        let sample_rate = 48_000; // Samples
        let gain_db = 6.0; // dB
        let filter = make_highshelf(frequency, sample_rate, gain_db, None);
        
        let target_vec = vec![2.2229172136088806, -3.9587208137297303, 1.7841414181566304,
                                      4.295432981120543, -7.922740859457287, 3.6756456963725253];

        let res_coeffs: Vec<&f64> = filter.a_coeffs.iter().chain(filter.b_coeffs.iter()).collect();        
        print_values(& target_vec, & res_coeffs);
        for i in 0..target_vec.len() {
            assert_eq!(*(res_coeffs[i]), target_vec[i]);
        }

        // assert_eq!(true, false);
    }

}
