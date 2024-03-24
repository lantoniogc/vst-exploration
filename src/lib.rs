use nih_plug::prelude::*;
use std::sync::Arc;

struct LowPassFilter {
    params: Arc<LowPassFilterParams>,
    alpha: f32,
    prev_output: f32,
}

#[derive(Params)]
struct LowPassFilterParams {
    #[id = "sample_rate"]
    pub sample_rate: FloatParam,
    #[id = "cutoff_freq"]
    pub cutoff_freq: FloatParam,
}

impl Default for LowPassFilter {
    fn default() -> Self {
        Self {
            params: Arc::new(LowPassFilterParams::default()),
            alpha: 0.0,
            prev_output: 0.0,
        }
    }
}

impl Default for LowPassFilterParams {
    fn default() -> Self {
        Self {
            sample_rate: FloatParam::new(
                "Sample Rate",
                44100.0,
                FloatRange::Linear { min: 1.0, max: 192000.0 },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(0))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            cutoff_freq: FloatParam::new(
                "Cutoff Frequency",
                420.0,
                FloatRange::Skewed {
                    min: 1.0,
                    max: 20_000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(0))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
        }
    }
}

impl LowPassFilter {
    fn update_coefficients(&mut self, sample_rate: f32, frequency: f32) {
        let dt = 1.0 / sample_rate;
        let rc = 1.0 / (2.0 * std::f32::consts::PI * frequency);
        self.alpha = dt / (dt + rc);
    }

    fn process_sample(&mut self, input: f32) -> f32 {
        self.prev_output = self.alpha * input + (1.0 - self.alpha) * self.prev_output;
        self.prev_output
    }
}

impl Plugin for LowPassFilter {
    const NAME: &'static str = "Low Pass Filter";
    const VENDOR: &'static str = "VSTRust";
    const URL: &'static str = "https://example.com";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[],
            aux_output_ports: &[],

            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let sample_rate = self.params.sample_rate.smoothed.next();
            let cutoff_freq = self.params.cutoff_freq.smoothed.next();
            self.update_coefficients(sample_rate, cutoff_freq);

            for sample in channel_samples {
                *sample = self.process_sample(*sample);
            }
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl ClapPlugin for LowPassFilter {
    const CLAP_ID: &'static str = "com.vst-rust.low-pass-filter";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A low pass filter");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for LowPassFilter {
    const VST3_CLASS_ID: [u8; 16] = *b"VSTRustLowPlugin";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(LowPassFilter);
nih_export_vst3!(LowPassFilter);