# Audio filters in Rust
A port of the WebAudio API filters to Rust.

## Description 
Audio or DSP filters, allow you to attenuate or accentuate some frequencies or range of frequencies in a signal. The signal can be of any kind, but in here, we will focus on 1D signals. Like audio signals. There can also occur differences in the signal phases, that vary with the filter and the frequency components of the signal. <br>
This is a port of Audio filters, from Python to Rust. My port was made from TheAlgorithms GitHub in Python (see references below). That is, by it self a port from WebAudio API C++ filters implementation used by the browsers. <br>


## The following filters are implemented over a BiQuad IIR filter
1. **low-pass**
2. **high-pass**
3. **band-pass**
4. **all-pass**
5. **peak**
6. **low-shelf**
7. **high-shelf** 
8. **notch**
9. **10 band equalizer**

## Plots of the filters Gain (dB) and phase shift response 

### low-pass - freq = 5.000 Hz - sample_rate = 48.000 samples/sec 

![Plot gain dB response](./plots/lowpass_gain.svg)
![Plot phase shift response](./plots/lowpass_phase.svg) <br>

### high-pass - freq = 5.000 Hz - sample_rate = 48.000 samples/sec

![Plot gain dB response](./plots/highpass_gain.svg)
![Plot phase shift response](./plots/highpass_phase.svg) <br>

### band-pass - freq = 10.000 Hz - sample_rate = 48.000 samples/sec - q_factor = 1.0

![Plot gain dB response](./plots/bandpass_gain.svg)
![Plot phase shift response](./plots/bandpass_phase.svg) <br>

### all-pass - freq = 10.000 Hz - sample_rate = 48.000 samples/sec

![Plot gain dB response](./plots/allpass_gain.svg)
![Plot phase shift response](./plots/allpass_phase.svg) <br>

### peak - freq = 10.000 Hz - sample_rate = 48.000 samples/sec - gain = 6 dB

![Plot gain dB response](./plots/peak_gain.svg)
![Plot phase shift response](./plots/peak_phase.svg) <br>

### peakEQ_const_Q G_pos - freq = 10.000 Hz - sample_rate = 48.000 samples/sec - gain = 5 dB
q_factor = 2 * sqrt(2) = 2.828 <br>

![Plot gain dB response](./plots/peak_eq_pos_g_gain.svg)
![Plot phase shift response](./plots/peak_eq_pos_g_phase.svg) <br>

### peakEQ_const_Q G_neg - freq = 10.000 Hz - sample_rate = 48.000 samples/sec - gain = -5 dB
q_factor = 2 * sqrt(2) = 2.828 <br>

![Plot gain dB response](./plots/peak_eq_neg_g_gain.svg)
![Plot phase shift response](./plots/peak_eq_neg_g_phase.svg) <br>

### low-shelf - freq = 10.000 Hz - sample_rate = 48.000 samples/sec - gain = 6 dB

![Plot gain dB response](./plots/lowshelf_gain.svg)
![Plot phase shift response](./plots/lowshelf_phase.svg) <br>

### high-shelf - freq = 10.000 Hz - sample_rate = 48.000 samples/sec - gain = 6 dB

![Plot gain dB response](./plots/highshelf_gain.svg)
![Plot phase shift response](./plots/highshelf_phase.svg) <br>

### notch - freq = 10.000 Hz - sample_rate = 48.000 samples/sec - q_factor = 0.05

![Plot gain dB response](./plots/notch_gain.svg)
![Plot phase shift response](./plots/notch_phase.svg) <br>

### 10 band Equalizer - sample_rate = 48.000 samples/sec - q_factor = 2.828
```
10 Band Equalizer

band_0:     29 Hz, gain: -10 dB
band_1:     59 Hz, gain:   0 dB
band_2:    119 Hz, gain:  -5 dB
band_3:    237 Hz, gain:   5 dB
band_4:    474 Hz, gain:   0 dB
band_5:    947 Hz, gain:  -5 dB
band_6:  1.889 Hz, gain:   0 dB
band_7:  3.770 Hz, gain:   5 dB
band_8:  7.523 Hz, gain:  10 dB
band_9: 15.011 Hz, gain:  12 dB
```

![Plot gain dB response](./plots/equalizer_10_band_gain.svg)
![Plot phase shift response](./plots/equalizer_10_band_phase.svg) <br>


## How to run the code 
```
To make a project for this files do:
    -Install Rust your computer (Linux, Win, Mac, Raspberry Pi).
     
    cargo new audio_filters_in_rust
    cd audio_filters_in_rust
     
    -Copy the repository files to this directory and overlap them.
 
To compile do:
    cargo build --release
 
To run do:
    cargo run --release
 
to run the tests do:
    cargo test
```


## References:

1. **GitHub - TheAlgorithms / Python / audio_filters** <br>
   [https://github.com/TheAlgorithms/Python/tree/master/audio_filters](https://github.com/TheAlgorithms/Python/tree/master/audio_filters)

2. **WebAudio - Cookbook formulae for audio equalizer biquad filter coefficients** <br>
   [https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html](https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html)

3. **Good resources on DSP** – Digital Signal Processing <br> 
   [https://github.com/joaocarvalhoopen/How_to_learn_modern_electronics#dsp--digital-signal-programming](https://github.com/joaocarvalhoopen/How_to_learn_modern_electronics#dsp--digital-signal-programming)

4. **Biquads - EarLevel** <br>
   [http://www.earlevel.com/main/2003/02/28/biquads/](http://www.earlevel.com/main/2003/02/28/biquads/)

5. **Biquad C++ source code - EarLevel** <br>
   [https://www.earlevel.com/main/2012/11/26/biquad-c-source-code/](https://www.earlevel.com/main/2012/11/26/biquad-c-source-code/)

6. **A biquad calculator v3 - EarLevel** <br>
   [https://www.earlevel.com/main/2021/09/02/biquad-calculator-v3/](https://www.earlevel.com/main/2021/09/02/biquad-calculator-v3/)

7. **WebAudio API - Mozilla Docs** <br>
   [https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API](https://developer.mozilla.org/pt-BR/docs/Web/API/Web_Audio_API)
  
8. **Audio Filters - Theory and Practice** <br>
   by Ethan Winer <br>
   [http://ethanwiner.com/filters.html](http://ethanwiner.com/filters.html)

9.  **Audio filter - Wikipedia** <br>
   [https://en.wikipedia.org/wiki/Audio_filter](https://en.wikipedia.org/wiki/Audio_filter)

11. **Electronic filter - Wikipedia** <br>
   [https://en.wikipedia.org/wiki/Electronic_filter](https://en.wikipedia.org/wiki/Electronic_filter)

**10 Bands Equalizer** <br>

12. **Making an EQ from cascading filters** <br>
    [https://dsp.stackexchange.com/questions/10309/making-an-eq-from-cascading-filters](https://dsp.stackexchange.com/questions/10309/making-an-eq-from-cascading-filters)
 
13. **Peak / Notch filter design** <br>
    [https://www.dsprelated.com/showcode/169.php](https://www.dsprelated.com/showcode/169.php)

14. **The Equivalence of Various Methods of Computing** <br>
    **Biquad Coefficients for Audio Parametric Equalizers** <br>
    [http://www.thesounddesign.com/MIO/EQ-Coefficients.pdf](http://www.thesounddesign.com/MIO/EQ-Coefficients.pdf)

15. **How to learn modern Rust** <br>
   [https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust](https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust)


## License

* **MIT Open Source License**, like the original license from <br>
  GitHub - TheAlgorithms / Python / audio_filters <br>
  [https://github.com/TheAlgorithms/Python/tree/master/audio_filters](https://github.com/TheAlgorithms/Python/tree/master/audio_filters)


## Have fun!
Best regards, <br>
Joao Nuno Carvalho

