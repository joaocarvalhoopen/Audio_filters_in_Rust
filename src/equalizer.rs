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
///                 -10 band equalizer
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
/// 
/// 10 Band Equalizer
/// 
///   12. Making an EQ from cascading filters
///       https://dsp.stackexchange.com/questions/10309/making-an-eq-from-cascading-filters
/// 
///   13. PEAK/NOTCH FILTER DESIGN
///       https://www.dsprelated.com/showcode/169.php
/// 
///   14. The Equivalence of Various Methods of Computing
///       Biquad Coefficients for Audio Parametric Equalizers
///       http://www.thesounddesign.com/MIO/EQ-Coefficients.pdf
///
///   15. How to learn modern Rust
///       https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust
///


use crate::iir_filter::ProcessingBlock; // Trait
use crate::iir_filter::IIRFilter;
use crate::butterworth_filter::make_peak_eq_constant_q;


pub struct Equalizer {
    sample_rate:     u32,
    bands_vec:       Vec<f64>,
    bands_gain_vec:  Vec<f64>,
    gain_max_db:     f64,
    gain_min_db:     f64,
    q_factor:        f64,
    iir_filters_vec: Vec<IIRFilter>,
}

impl Equalizer {
    pub fn new(sample_rate: u32, bands_vec: & Vec<f64>,
           gain_max_db:f64, gain_min_db:f64,
           q_factor:f64
           ) -> Self {
        let mut equalizer = Equalizer{
            sample_rate,
            bands_vec: bands_vec.clone(),
            bands_gain_vec: vec![0.0; bands_vec.len()],
            gain_max_db,
            gain_min_db,
            q_factor,
            iir_filters_vec: Vec::with_capacity(bands_vec.len())
        };
        equalizer.gen_chain_filters();

        equalizer
    }

    fn gen_chain_filters(& mut self) {
        for band in & self.bands_vec {
            let frequency_center = *band;
            let gain_db = 0.0;   // dB
            let iir_filter = make_peak_eq_constant_q(frequency_center, self.sample_rate, gain_db, Some(self.q_factor));
            self.iir_filters_vec.push(iir_filter); 
        }
    }

    fn change_filter(& mut self, index: usize) {
        assert!(index < self.bands_vec.len());
        let frequency_center = self.bands_vec[index];
        let gain_db = self.bands_gain_vec[index];   // dB
        let q_factor = Some(self.q_factor);
        // NOTE: Correcting factor with frequency.
        // let q_factor = Some(self.q_factor + /*0.4*/ 0.6 * (self.bands_gain_vec.len() - index - 1) as f64);
        let iir_filter_tmp = make_peak_eq_constant_q(frequency_center, self.sample_rate, gain_db, q_factor);
        // This will probably make an abrupt change to the sound, so we are not losing the internal buffer samples. 
        //   self.iir_filters_vec[index] = iir_filter;
        // We generated the correct new coefficients in a new temporary filter and
        // now we are applying to the actual filter that is in the filter chain,
        // only changing the coefficients.
        let _ = self.iir_filters_vec[index].set_coefficients(& iir_filter_tmp.a_coeffs, & iir_filter_tmp.b_coeffs);
    }

    pub fn get_bands_freq(& self, index: usize) -> f64 {
        assert!(index < self.bands_vec.len());
        self.bands_vec[index]
    }

    pub fn get_band_gain(& self, index: usize) -> f64 {
        assert!(index < self.bands_vec.len());
        self.bands_gain_vec[index]
    }

    pub fn set_band_gain(& mut self, index: usize, gain_db: f64) -> Result<(), String> {
        assert!(index < self.bands_vec.len());
        if gain_db < self.gain_min_db || gain_db > self.gain_max_db {
            return Err(format!("Error: invalid gain value {}, must be in the interval [{}, {}]",
                       gain_db, self.gain_min_db, self.gain_max_db));
        }
        self.bands_gain_vec[index] = gain_db;
        self.change_filter(index);
        
        Ok(())
    }

    pub fn make_equalizer_10_band(sample_rate: u32) -> Equalizer {
        // Note: My Q_factor is correct for a octave, that means that the frequency between bands
        //       has to double in each band, but where can I now the standard values where to start
        //       the band_0, so that I can double after that, I got the frequencies from here:
        //          Gstreamer 10 band equalizer plugin.
        //          https://gitlab.freedesktop.org/gstreamer/gst-plugins-good/-/blob/086bad464387d61e31884ee6628846628118fbcb/gst/equalizer/gstiirequalizer10bands.c  
        let bands_vec = vec![
            29.0,    // Hz band_0
            59.0,    // Hz band_1
            119.0,   // Hz band_2
            237.0,   // Hz band_3
            474.0,   // Hz band_4
            947.0,   // Hz band_5
            1889.0,  // Hz band_6
            3770.0,  // Hz band_7
            7523.0,  // Hz band_8
            15011.0  // Hz band_9
        ];

        let gain_max_db  =  12.0; // dB
        let gain_min_db  = -24.0; // dB
        // let gain_center_db =   0.0; // dB

        // A good value for a 10 band equalizer.
        // See: The second reference on the function make_peak_eq_constant_q.
        let q_factor = 2.0 * f64::sqrt(2.0);  // ~ 2.828

        let equalizer_10_band = Equalizer::new(sample_rate, & bands_vec, gain_max_db, gain_min_db, q_factor);

        equalizer_10_band
    }

}

impl ProcessingBlock for Equalizer {
    fn process(& mut self, sample: f64) -> f64 {
        let mut sample_t =  sample;
        for iir_filter in & mut self.iir_filters_vec {
            sample_t = iir_filter.process(sample_t);
        }

        sample_t
    }
}
