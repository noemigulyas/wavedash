use {rodio::Source, std::time::Duration};

pub struct SampleBuffer {
    samples: Vec<f32>,
    current_index: usize,
}

impl From<Vec<f32>> for SampleBuffer {
    fn from(samples: Vec<f32>) -> Self {
        Self {
            samples,
            current_index: 0,
        }
    }
}

impl Iterator for SampleBuffer {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.samples.len() {
            return None;
        }

        let sample = self.samples[self.current_index];
        self.current_index += 1;

        Some(sample)
    }
}

impl Source for SampleBuffer {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        1
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        44100
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
