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


// Module definition
mod iir_filter;
mod butterworth_filter;
mod show_response;
mod equalizer;

// Imports
use crate::iir_filter::ProcessingBlock;  // Trait
use crate::iir_filter::IIRFilter;
use crate::butterworth_filter::make_lowpass;
use crate::butterworth_filter::make_highpass;
use crate::butterworth_filter::make_bandpass;
use crate::butterworth_filter::make_allpass;
use crate::butterworth_filter::make_peak;
use crate::butterworth_filter::make_peak_eq_constant_q;
use crate::butterworth_filter::make_lowshelf;
use crate::butterworth_filter::make_highshelf;
use crate::butterworth_filter::make_notch;

use crate::show_response::show_frequency_response;
use crate::show_response::show_phase_response;

use crate::equalizer::Equalizer;


fn main() {
    println!("***************************");
    println!("** Audio filters in Rust **");
    println!("***************************");

    test_a();
    test_b();

    generate_plots();
    // generate_plot_equalizer_10_bands_01();
    generate_plot_equalizer_10_bands_02();
}

fn test_a() {
    let mut filter = IIRFilter::new(2);
    let res = filter.process(0.0);
    println!("filter res: {} should be 0.0 .", res);
}

fn test_b() {
    let frequency = 200.0; // Hz
    let sample_rate = 44100; // Hz
    let mut filter = make_lowpass(frequency, sample_rate, None);
    let sample = 0.0;
    let res = filter.process(sample);

    println!("filter res: {} should be ?? .", res);
}

fn generate_plots() {
    print!("\nStarting generating the SVG plots...");

    // low-pass
    let frequency   = 5_000.0;  // Hz
    let sample_rate = 48_000;   // Samples
    let mut filter = make_lowpass(frequency, sample_rate, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/lowpass_gain.svg", "lowpass");
    show_phase_response(& mut filter, sample_rate as usize, "plots/lowpass_phase.svg", "lowpass");

    // high-pass
    let frequency   = 5_000.0;  // Hz
    let sample_rate = 48_000;   // Samples
    let mut filter = make_highpass(frequency, sample_rate, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/highpass_gain.svg", "highpass");
    show_phase_response(& mut filter, sample_rate as usize, "plots/highpass_phase.svg", "highpass");

    // band-pass
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000;    // Samples
    // Note: I have put a larger q_factor then the default so that the band pass is more accentuated. 
    let q_factor = Some(1.0);
    let mut filter = make_bandpass(frequency, sample_rate, q_factor);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/bandpass_gain.svg", "bandpass");
    show_phase_response(& mut filter, sample_rate as usize, "plots/bandpass_phase.svg", "bandpass");

    // all-pass
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000;    // Samples
    let mut filter = make_allpass(frequency, sample_rate, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/allpass_gain.svg", "allpass");
    show_phase_response(& mut filter, sample_rate as usize, "plots/allpass_phase.svg", "allpass");

    // peak
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000;    // Samples
    let gain_db     = 6.0;       // dB
    let mut filter = make_peak(frequency, sample_rate, gain_db, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/peak_gain.svg", "peak");
    show_phase_response(& mut filter, sample_rate as usize, "plots/peak_phase.svg", "peak");

    // peak_eq_constant_q positive and negative gain.
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000;    // Samples
    let gain_db     = 5.0;       // dB
    // A good value for a 10 band equalizer.
    // See: The second reference on the function make_peak_eq_constant_q.
    let q_factor = Some(2.0 * f64::sqrt(2.0));
    let mut filter = make_peak_eq_constant_q(frequency, sample_rate, gain_db, q_factor);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/peak_eq_pos_g_gain.svg", "peakEQ_G+");
    show_phase_response(& mut filter, sample_rate as usize, "plots/peak_eq_pos_g_phase.svg", "peakEQ_G+");
    let gain_db     = -5.0;       // dB
    let mut filter = make_peak_eq_constant_q(frequency, sample_rate, gain_db, q_factor);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/peak_eq_neg_g_gain.svg", "peakEQ_G-");
    show_phase_response(& mut filter, sample_rate as usize, "plots/peak_eq_neg_g_phase.svg", "peakEQ_G-");

    // low-shelf
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000;    // Samples
    let gain_db     = 6.0;       // dB
    let mut filter = make_lowshelf(frequency, sample_rate, gain_db, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/lowshelf_gain.svg", "lowshelf");
    show_phase_response(& mut filter, sample_rate as usize, "plots/lowshelf_phase.svg", "lowshelf");

    // high-shelf
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000;    // Samples
    let gain_db     = 6.0;       // dB
    let mut filter = make_highshelf(frequency, sample_rate, gain_db, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/highshelf_gain.svg", "highshelf");
    show_phase_response(& mut filter, sample_rate as usize, "plots/highshelf_phase.svg", "highshelf");

    // notch
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000;    // Samples
    let q_factor    = 0.05;
    let mut filter = make_notch(frequency, sample_rate, Some(q_factor));
    show_frequency_response(& mut filter, sample_rate as usize, "plots/notch_gain.svg", "notch");
    show_phase_response(& mut filter, sample_rate as usize, "plots/notch_phase.svg", "notch");

    println!("\n ... ended generating the SVG plots.");
}

#[allow(dead_code)]
fn generate_plot_equalizer_10_bands_01() {
    println!("\n10 Band Equalizer\n");
    let sample_rate = 48_000;
    let mut eq: Equalizer = Equalizer::make_equalizer_10_band(sample_rate);
    // Set the gains for each_frequency band.
    let _= eq.set_band_gain(0, -15.0);
    let _= eq.set_band_gain(2, -10.0);
    let _= eq.set_band_gain(1,  -5.0);
    let _= eq.set_band_gain(3,   0.0);
    let _= eq.set_band_gain(4,  -5.0);
    let _= eq.set_band_gain(5,  10.0);
    let _= eq.set_band_gain(6, -15.0);
    let _= eq.set_band_gain(7,   0.0);
    let _= eq.set_band_gain(8,   5.0);
    let _= eq.set_band_gain(9, -10.0);
    for i in 0..10 {
        println!("{} Hz :  {} dB", eq.get_bands_freq(i), eq.get_band_gain(i));
    }
    println!("\n");
    show_frequency_response(& mut eq, sample_rate as usize, "plots/equalizer_10_band_gain.svg", "equ_10_bands");
    show_phase_response(& mut eq, sample_rate as usize, "plots/equalizer_10_band_phase.svg", "equ_10_bands");
}

fn generate_plot_equalizer_10_bands_02() {
    println!("\n10 Band Equalizer\n");
    let sample_rate = 48_000;
    let mut eq: Equalizer = Equalizer::make_equalizer_10_band(sample_rate);
    // Set the gains for each_frequency band.
    let _= eq.set_band_gain(0, -10.0);
    let _= eq.set_band_gain(2,  -5.0);
    let _= eq.set_band_gain(1,   0.0);
    let _= eq.set_band_gain(3,   5.0);
    let _= eq.set_band_gain(4,   0.0);
    let _= eq.set_band_gain(5,  -5.0);
    let _= eq.set_band_gain(6,   0.0);
    let _= eq.set_band_gain(7,   5.0);
    let _= eq.set_band_gain(8,  10.0);
    let _= eq.set_band_gain(9,  12.0);
    for i in 0..10 {
        println!("{} Hz :  {} dB", eq.get_bands_freq(i), eq.get_band_gain(i));
    }
    println!("\n");
    show_frequency_response(& mut eq, sample_rate as usize, "plots/equalizer_10_band_gain.svg", "equ_10_bands");
    show_phase_response(& mut eq, sample_rate as usize, "plots/equalizer_10_band_phase.svg", "equ_10_bands");
}
