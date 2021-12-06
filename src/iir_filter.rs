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
///                 -low-pass
///                 -high-pass
///                 -band-pass
///                 -all-pass
///                 -peak
///                 -low-shelf
///                 -high-shelf 
///                 -notch
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
///    2. WebAudio - Cookbook formulae for audio equalizer biquad filter coefficients
///       https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html 
/// 
///    3. Good resources on DSP – Digital Signal Programming
///       https://github.com/joaocarvalhoopen/How_to_learn_modern_electronics#dsp--digital-signal-programming
///
///    4. Biquads - EarLevel
///       http://www.earlevel.com/main/2003/02/28/biquads/
///
///    5. Biquad C++ source code - EarLevel
///       https://www.earlevel.com/main/2012/11/26/biquad-c-source-code/
///
///    6. A biquad calculator V3 - EarLevel
///       https://www.earlevel.com/main/2021/09/02/biquad-calculator-v3/
/// 
///    7. WebAudio API - Mozilla Docs
///       https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API
/// 
///    8. Audio Filters - Theory and Practice
///       by Ethan Winer
///       http://ethanwiner.com/filters.html
/// 
///    9. Audio filter - Wikipedia
///       https://en.wikipedia.org/wiki/Audio_filter
/// 
///   10. Electronic filter - Wikipedia
///       https://en.wikipedia.org/wiki/Electronic_filter
///
///   11. How to learn modern Rust
///       https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust
///


/// N-Order IIR filter
/// Assumes working with float samples normalized on [-1, 1]
///
/// Implementation details:
///    Based on the 2nd-order function from
///    https://en.wikipedia.org/wiki/Digital_biquad_filter,
///    this generalized N-order function was made.
///
/// Using the following transfer function
///   H(z)=\frac{b_{0}+b_{1}z^{-1}+b_{2}z^{-2}+...+b_{k}z^{-k}}{a_{0}+a_{1}z^{-1}+a_{2}z^{-2}+...+a_{k}z^{-k}}
/// we can rewrite this to
///   y[n]={\frac{1}{a_{0}}}\left(\left(b_{0}x[n]+b_{1}x[n-1]+b_{2}x[n-2]+...+b_{k}x[n-k]\right)-\left(a_{1}y[n-1]+a_{2}y[n-2]+...+a_{k}y[n-k]\right)\right)
///
pub struct IIRFilter {
    pub order: usize,
    // a_{0} ... a_{k}
    pub a_coeffs: Vec<f64>,
    // b_{0} ... b_{k}
    pub b_coeffs: Vec<f64>,
    // x[n-1] ... x[n-k]
    input_history: Vec<f64>,
    // y[n-1] ... y[n-k]
    output_history: Vec<f64>,
}

impl IIRFilter {
    pub fn new(order: usize) -> Self {
        IIRFilter {
            order: order,
            // a_{0} ... a_{k}
            a_coeffs: { let mut a_coeffs = vec![0.0; 1 + order];
                        a_coeffs[0] = 1.0;
                        a_coeffs },
            // b_{0} ... b_{k}
            b_coeffs: { let mut b_coeffs = vec![0.0; 1 + order];
                         b_coeffs[0] = 1.0;
                         b_coeffs },
            // x[n-1] ... x[n-k]
            input_history: vec![0.0; order],
            // y[n-1] ... y[n-k]
            output_history: vec![0.0; order],
        }
    }

    /// Set the coefficients for the IIR filter. These should both be of size order + 1.
    /// a_0 may be left out, and it will use 1.0 as default value.
    ///
    /// This method works well with scipy's filter design functions
    ///    >>> # Make a 2nd-order 1000Hz butterworth lowpass filter
    ///    >>> import scipy.signal
    ///    >>> b_coeffs, a_coeffs = scipy.signal.butter(2, 1000,
    ///    ...                                          btype='lowpass',
    ///    ...                                          fs=48000)
    ///    >>> filt = IIRFilter(2)
    ///    >>> filt.set_coefficients(a_coeffs, b_coeffs)
    ///
    /// In Rust
    ///    >>> let a_coeffs = [0.1,  0.2,  0.3]
    ///    >>> let b_coeffs = [0.15, 0.25, 0.35]
    ///    >>> let filter_order: u32 = 2;
    ///    >>> let iir_filter = IIR_Filter::new(filter_order);
    ///    >>> iir_filter.set_coefficients(& a_coeffs[], & b_coeffs[]);
    ///          
    pub fn set_coefficients(& mut self, a_coeffs: &[f64], b_coeffs: &[f64]) -> Result<(), String> {
        if a_coeffs.len() != self.order + 1 && a_coeffs.len() != self.order {
            return Err(
                     r"Expected a_coeffs to have {self.order + 1} elements for {self.order} /
                       -order filter, got {len(a_coeffs)}".to_string());
        }
        if b_coeffs.len() != self.order + 1 {
            return Err(
                     r"Expected b_coeffs to have {self.order + 1} elements for {self.order} /
                     -order filter, got {len(a_coeffs)}".to_string());
        }
        self.a_coeffs.clear();
        if a_coeffs.len() < self.order + 1 {
            self.a_coeffs.push(1.0);
            self.a_coeffs.extend(a_coeffs);
        } else {
            self.a_coeffs.extend(a_coeffs);
        }
        self.b_coeffs.clear();
        self.b_coeffs.extend(b_coeffs);
        
        Ok(())
    }
    
    /// Calculate y[n]
    /// 
    /// In Python
    ///     >>> filt = IIRFilter(2)
    ///     >>> filt.process(0)
    ///     0.0
    /// 
    /// In Rust
    ///     >>> let filt = IIRFilter::new(2)
    ///     >>> filt.process(0.0)
    ///     0.0
    ///
    pub fn process(& mut self, sample: f64) -> f64 {
        let mut result: f64 = 0.0;

        // Start at index 1 and do index 0 at the end.
        for i in 1..(self.order + 1) {
            result +=   self.b_coeffs[i] * self.input_history[i - 1]
                      - self.a_coeffs[i] * self.output_history[i - 1];
        }
    
        result = (result + self.b_coeffs[0] * sample) / self.a_coeffs[0];

        let input_len  = self.input_history.len();
        let output_len = self.output_history.len();
        self.input_history.copy_within(0..(input_len - 1), 1);
        self.output_history.copy_within(0..(output_len - 1), 1);

        self.input_history[0]  = sample;
        self.output_history[0] = result;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iir_filter_000() {

        let mut filter = IIRFilter::new(2);
        let res = filter.process(0.0);
        assert!((res - 0.0).abs() < 0.00001);

        println!("filter res: {} , should be 0.0 .", res);
        // assert_eq!(true, false);
    }

    #[test]
    fn test_iir_filter_001() {
        // 1º case.
        let a_coeffs = [0.0, 0.0];
        let b_coeffs = [0.0, 0.0, 0.0];
        let filter_order: usize = 2;
        let mut filter = IIRFilter::new(filter_order);
        let res_coef = filter.set_coefficients(& a_coeffs, & b_coeffs);
        assert!(res_coef.is_ok());
        let res = filter.process(0.0);
        assert!((res - 0.0).abs() < 0.00001);

        println!("filter res: {} , should be 0.0 .", res);

        // 2º case.
        let a_coeffs = [1.0, 0.0, 0.0];
        let b_coeffs = [0.0, 0.0, 0.0];
        let filter_order: usize = 2;
        let mut filter = IIRFilter::new(filter_order);
        let res_coef = filter.set_coefficients(& a_coeffs, & b_coeffs);
        assert!(res_coef.is_ok());
        let res = filter.process(0.0);
        assert!((res - 0.0).abs() < 0.00001);

        println!("filter res: {} , should be 0.0 .", res);

        // assert_eq!(true, false);
    }

}

