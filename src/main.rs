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
///    2. WebAudio - Cookbook formulae for audio equalizer biquad filter coefficients
///       https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html 
/// 
///    3. Good resources on DSP – Digital Signal Programming
///       https://github.com/joaocarvalhoopen/How_to_learn_modern_electronics#dsp--digital-signal-programming
///
///    4. WebAudio API - Mozilla Docs
///       https://developer.mozilla.org/pt-BR/docs/Web/API/Web_Audio_API
/// 
///    5. Audio Filters - Theory and Practice
///       by Ethan Winer
///       http://ethanwiner.com/filters.html
/// 
///    6. Audio filter - Wikipedia
///       https://en.wikipedia.org/wiki/Audio_filter
/// 
///    7. Electronic filter - Wikipedia
///       https://en.wikipedia.org/wiki/Electronic_filter
///
///    8. How to learn modern Rust
///       https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust
/// 


mod iir_filter;
mod butterworth_filter;
mod show_response;

use crate::iir_filter::IIRFilter;
use crate::butterworth_filter::make_lowpass;
use crate::butterworth_filter::make_highpass;
use crate::butterworth_filter::make_bandpass;
use crate::butterworth_filter::make_allpass;
use crate::butterworth_filter::make_peak;
use crate::butterworth_filter::make_lowshelf;
use crate::butterworth_filter::make_highshelf;

use crate::show_response::show_frequency_response;
use crate::show_response::show_phase_response;


fn main() {
    println!("***************************");
    println!("** Audio filters in Rust **");
    println!("***************************");

    test_a();
    test_b();

    generate_plots();
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

    // lowpass
    let frequency   = 5_000.0;  // Hz
    let sample_rate = 48_000; // Samples
    let mut filter = make_lowpass(frequency, sample_rate, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/lowpass_gain.svg", "lowpass");
    show_phase_response(& mut filter, sample_rate as usize, "plots/lowpass_phase.svg", "lowpass");

    // highpass
    let frequency   = 5_000.0;  // Hz
    let sample_rate = 48_000; // Samples
    let mut filter = make_highpass(frequency, sample_rate, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/highpass_gain.svg", "highpass");
    show_phase_response(& mut filter, sample_rate as usize, "plots/highpass_phase.svg", "highpass");

    // bandpass
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000;  // Samples
    // Note: I have put a larger q_factor then the default so that the band pass is more accentuated. 
    let q_factor = Some(1.0);
    let mut filter = make_bandpass(frequency, sample_rate, q_factor);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/bandpass_gain.svg", "bandpass");
    show_phase_response(& mut filter, sample_rate as usize, "plots/bandpass_phase.svg", "bandpass");

    // allpass
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000; // Samples
    let mut filter = make_allpass(frequency, sample_rate, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/allpass_gain.svg", "allpass");
    show_phase_response(& mut filter, sample_rate as usize, "plots/allpass_phase.svg", "allpass");

    // peak
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000; // Samples
    let gain_db = 6.0;          // dB
    let mut filter = make_peak(frequency, sample_rate, gain_db, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/peak_gain.svg", "peak");
    show_phase_response(& mut filter, sample_rate as usize, "plots/peak_phase.svg", "peak");

    // lowshelf
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000; // Samples
    let gain_db = 6.0;          // dB
    let mut filter = make_lowshelf(frequency, sample_rate, gain_db, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/lowshelf_gain.svg", "lowshelf");
    show_phase_response(& mut filter, sample_rate as usize, "plots/lowshelf_phase.svg", "lowshelf");

    // highshelf
    let frequency   = 10_000.0;  // Hz
    let sample_rate = 48_000; // Samples
    let gain_db = 6.0;          // dB
    let mut filter = make_highshelf(frequency, sample_rate, gain_db, None);
    show_frequency_response(& mut filter, sample_rate as usize, "plots/highshelf_gain.svg", "highshelf");
    show_phase_response(& mut filter, sample_rate as usize, "plots/highshelf_phase.svg", "highshelf");

    println!("\n ... ended generating the SVG plots.");

}

