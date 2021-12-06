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



use crate::iir_filter::IIRFilter;
use std::f32::consts::TAU as TAU_f32;


/// Get bounds for printing fft results
/// 
/// In Python:
///     >>> import numpy
///     >>> array = numpy.linspace(-20.0, 20.0, 1000)
///     >>> get_bounds(array, 1000)
///     (-20, 20)
///
pub fn get_bounds(fft_results: & [f32], sample_rate: usize, x_bound_max: usize) -> (f32, f32) {
    // let slice_upper_bound = (sample_rate / 2) - 1;
    // let slice_upper_bound = (sample_rate / 2) - 1 - 100;
    let slice_upper_bound = x_bound_max;
    // This will remove the bounds checks from the array at each access.
    assert!(slice_upper_bound <= fft_results.len());
    let mut min_t = -20.0;  // f64::MAX;
    let mut max_t =  20.0;  // f64::MIN;
    for i in 1..slice_upper_bound{
        min_t = f32::min(fft_results[i], min_t);
        max_t = f32::max(fft_results[i], max_t);
    }
    let lowest = min_t;
    let highest = max_t;
    
    (lowest, highest)
} 

/// Show frequency response of a filter
///
/// In Python:
///     >>> from audio_filters.iir_filter import IIRFilter
///     >>> filt = IIRFilter(4)
///     >>> show_frequency_response(filt, 48000)
///
pub fn show_frequency_response(filter: & mut IIRFilter, sample_rate: usize, path: & str, line_name: & str) {

    let size = 512_usize;
    // Excites the filter with an input of only a peak value (1.0) in the first sample, and the rest with (0.0) zero, as samples.
    // It's a Dirac Impulse. 
    let inputs = { let mut inputs = vec![0.0; size - 1 + 1];
                            inputs[0] = 1.0;  
                            inputs
                          };
    let mut outputs: Vec<f64> = Vec::with_capacity(size);
    for i in 0..size {
        outputs.push(filter.process(inputs[i]));
    }
    // zero-padding.
    let filler = vec![0.0; sample_rate - size];
    outputs.extend(filler);

    // Perform a forward FFT of size 1234
    use rustfft::{FftPlanner, num_complex::Complex};

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(sample_rate);

    let mut buffer = vec![Complex{ re: 0.0_f32, im: 0.0_f32 }; sample_rate];

    for i in 0..outputs.len() {
        buffer[i].re = outputs[i] as f32;
    }

    fft.process(& mut buffer[..]);

    // Calculates the absolute value or the norm. 
    let fft_out = buffer.iter().map(|c| c.norm() ).collect::<Vec<f32>>();
    // Transform the result into dB's.
    let fft_db = fft_out.iter().map(|val| 20.0 * f32::log10(*val) ).collect::<Vec<f32>>();


    // Display within reasonable bounds
    let (x_bound_min, x_bound_max) = (0_usize, sample_rate / 2 - 1 - 100 );
    let fft_db = & fft_db[x_bound_min..x_bound_max];
    let bounds = get_bounds(& fft_db, sample_rate, x_bound_max);
    let (y_bound_min, y_bound_max) = (f32::max(-80.0, bounds.0), f32::min(80.0, bounds.1) );

    // Frequencies on log scale from 24 to nyquist frequency
    use plotters::prelude::*;
    //fn main() -> Result<(), Box<dyn std::error::Error>> {
        let root = SVGBackend::new(path /* "plots/0.svg" */, (400, 300)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption(line_name.to_string() + " - Gain(dB) vs Freq", ("sans-serif", 25).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_bound_min..x_bound_max, y_bound_min..y_bound_max )
            .unwrap();
    
        chart.configure_mesh().draw().unwrap();
    
        chart
            .draw_series(LineSeries::new(
                fft_db.iter().enumerate().map(|pair| (pair.0, *pair.1 ) ),
                &BLUE,
            )).unwrap()
            .label(line_name)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw().unwrap();
}

/// Show phase response of a filter
/// 
/// In Python:
///     >>> from audio_filters.iir_filter import IIRFilter
///     >>> filt = IIRFilter(4)
///     >>> show_phase_response(filt, 48000)
/// 
pub fn show_phase_response(filter: & mut IIRFilter, sample_rate: usize, path: & str, line_name: & str) {

    let size = 512_usize;
    // Excites the filter with an input of only a peak value (1.0) in the first sample, and the rest with (0.0) zero, as samples.
    // It's a Dirac Impulse. 
    let inputs = { let mut inputs = vec![0.0; size - 1 + 1];
                            inputs[0] = 1.0;  
                            inputs
                          };
    let mut outputs: Vec<f64> = Vec::with_capacity(size);
    for i in 0..size {
        outputs.push(filter.process(inputs[i]));
    }
    // zero-padding.
    let filler = vec![0.0; sample_rate - size];
    outputs.extend(filler);

    // Perform a forward FFT of size 1234
    use rustfft::{FftPlanner, num_complex::Complex};

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(sample_rate);

    let mut buffer = vec![Complex{ re: 0.0_f32, im: 0.0_f32 }; sample_rate];

    for i in 0..outputs.len() {
        buffer[i].re = outputs[i] as f32;
    }

    fft.process(& mut buffer[..]);

    // Calculates the phase angle or the atan(b/a) for a complex number c = a + bj . 
    // let fft_out = buffer.iter().map(|c| c.atan().re ).collect::<Vec<f32>>();
    let fft_out = buffer.iter().map(|c| f32::atan2(c.im, c.re) ).collect::<Vec<f32>>();

    // Display within reasonable bounds
    let (x_bound_min, x_bound_max) = (0_usize, sample_rate / 2 - 1 - 150     );
    let fft_out = & fft_out[x_bound_min..x_bound_max];
    let bounds = get_bounds(& fft_out, sample_rate, x_bound_max);
    // let (y_bound_min, y_bound_max) = (f32::max(-80.0, bounds.0), f32::min(80.0, bounds.1) );
    // NOTE: Remember that TAU = 2 * PI.
    let (y_bound_min, y_bound_max) = (f32::max(-TAU_f32, bounds.0), f32::min(TAU_f32, bounds.1) );

    // Frequencies on log scale from 24 to nyquist frequency
    use plotters::prelude::*;
    //fn main() -> Result<(), Box<dyn std::error::Error>> {
        let root = SVGBackend::new(path /* "plots/0.svg" */, (400, 300)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption(line_name.to_string() + " - Phase shift(Rad) vs Freq", ("sans-serif", 25).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_bound_min..x_bound_max, y_bound_min..y_bound_max )
            .unwrap();
    
        chart.configure_mesh().draw().unwrap();
    
        chart
            .draw_series(LineSeries::new(
                fft_out.iter().enumerate().map(|pair| (pair.0, *pair.1 ) ),
                &BLUE,
            )).unwrap()
            .label(line_name)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::butterworth_filter::make_lowpass;

    #[test]
    fn test_show_frequency_response() {
        let frequency = 5_000.0;  // Hz
        let sample_rate = 48_000; // Samples
        let mut filter = make_lowpass(frequency, sample_rate, None);
        // show_frequency_response(& mut filter, sample_rate as usize, "plots/lowpass.svg", "lowpass");
        
        // assert_eq!(true, false);
    }
}

