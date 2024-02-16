// Equalizer is a struct that represents FFMPEG's equalizer filter.
// ex: equalizer=f=1000:t=q:w=1:g=2,equalizer=f=100:t=q:w=2:g=-5

use std::vec;

use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy)]
pub enum Equalizer {
    RadioEqualizer,
    RockEqualizer,
    PopEqualizer,
    JazzEqualizer,
}

#[derive(Clone, Debug)]
pub struct EqualizerFilter {
    bands: Vec<EqualizerBand>,
    pub name: Equalizer,
}

#[derive(Debug, Clone, PartialEq)]
struct EqualizerBand {
    frequency: f32,
    width_type: String, // q, h, o, s, k
    width: f32,
    gain: f32,
}

impl EqualizerFilter {
    pub fn get_filter(&self) -> Vec<String> {
        let default_params = [
            "-f",
            "s16le",
            "-ac",
            "2",
            "-ar",
            "48000",
            "-b:a",
            "8k",
            "-acodec",
            "pcm_f32le",
        ]
        .map(|s| s.to_string());

        let mut params = default_params.to_vec();
        let mut equalizer_params = vec![];

        if self.bands.is_empty() {
            return params;
        }

        params.push("-af".to_string());

        for band in &self.bands {
            let formatted_string = format!(
                "equalizer=f={}:t={}:w={}:g={}",
                band.frequency, band.width_type, band.width, band.gain
            );

            equalizer_params.push(formatted_string);
        }

        params.push(
            // resample + equalizer
            format!(
                "aresample=8000:resampler=soxr:precision=33:osf=s16:dither_method=triangular,{}",
                equalizer_params.join(",")
            ),
        );

        params.push("-".to_string());

        params
    }
}

impl Equalizer {
    pub fn get_filter(&self) -> EqualizerFilter {
        match self {
            Equalizer::RadioEqualizer => RADIO_EQUALIZER.clone(),
            _ => RADIO_EQUALIZER.clone(),
        }
    }
}

impl std::fmt::Display for Equalizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Equalizer::RadioEqualizer => write!(f, "Radio Equalizer"),
            Equalizer::RockEqualizer => write!(f, "Rock Equalizer"),
            Equalizer::PopEqualizer => write!(f, "Pop Equalizer"),
            Equalizer::JazzEqualizer => write!(f, "Jazz Equalizer"),
        }
    }
}

// ------
// RadioEqualizer
// ------
// Bands are based on the following:
// 20Hz   = - 30dB
// 35Hz   = - 30dB
// 40Hz   = - 12dB
// 55Hz   = - 3dB
// 60Hz   =   0dB
// 4000Hz =   0dB
// 5200Hz = - 3dB

pub static RADIO_EQUALIZER: Lazy<EqualizerFilter> = Lazy::new(|| EqualizerFilter {
    name: Equalizer::RadioEqualizer,
    bands: vec![
        EqualizerBand {
            frequency: 20.0,
            width_type: String::from("h"),
            width: 1.0,
            gain: -30.0,
        },
        EqualizerBand {
            frequency: 35.0,
            width_type: String::from("h"),
            width: 1.0,
            gain: -30.0,
        },
        EqualizerBand {
            frequency: 40.0,
            width_type: String::from("h"),
            width: 1.0,
            gain: -12.0,
        },
        EqualizerBand {
            frequency: 55.0,
            width_type: String::from("h"),
            width: 1.0,
            gain: -3.0,
        },
        EqualizerBand {
            frequency: 60.0,
            width_type: String::from("h"),
            width: 1.0,
            gain: 0.0,
        },
        EqualizerBand {
            frequency: 4000.0,
            width_type: String::from("h"),
            width: 1.0,
            gain: 0.0,
        },
        EqualizerBand {
            frequency: 5200.0,
            width_type: String::from("h"),
            width: 1.0,
            gain: -3.0,
        },
    ],
});
