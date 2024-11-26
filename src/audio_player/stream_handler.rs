use cpal::{Device, Sample, SampleFormat, SizedSample, Stream, StreamConfig, FromSample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct StreamHandler {
    stream: Stream,
    total_samples: usize,
    cursor: Arc<AtomicUsize>,
}

impl StreamHandler {
    pub fn from_samples<T>(samples: Vec<T>) -> Self
    where
    T: Sample + Send + 'static,    
    u8 : FromSample<T>,
    u16: FromSample<T>,
    u32: FromSample<T>,
    u64: FromSample<T>,
    i8 : FromSample<T>,
    i16: FromSample<T>,
    i32: FromSample<T>,
    i64: FromSample<T>,
    f32: FromSample<T>,
    f64: FromSample<T>, {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        
        let mut supported_configs_range = device.supported_output_configs()
            .expect("error while querying configs");

        let supported_config = supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();

        let config = supported_config.config();
        let sample_format = supported_config.sample_format();
        
        let total_samples = samples.len();
        let cursor = Arc::new(AtomicUsize::new(0));

        let stream_cursor = Arc::clone(&cursor);
        let stream = match sample_format {
            SampleFormat::I16 => build_stream::<T, i16>(device, config, samples, stream_cursor),
            SampleFormat::U8 => build_stream::<T, u8>(device, config, samples, stream_cursor),
            sample_format => panic!("Unsupported sample format '{sample_format}'")
        };

        stream.play().expect("Should be able to play stream"); //Currently choosing to start the stream when it is created

        Self {
            stream: stream,
            total_samples,
            cursor,
        }
    }

    pub fn play(&self) {
        self.stream.play().expect("Should be able to play stream");
    }
    
    pub fn pause(&self) {
        self.stream.pause().expect("Should be able to pause stream"); //Probably shouldn't crash in the case that a devices doesn't support pausing the stream
    }

    pub fn progress(&self) -> f32 {
        self.cursor.load(Ordering::Relaxed) as f32 / self.total_samples as f32
    }

    pub fn restart(&self) {
        self.cursor.store(0, Ordering::Relaxed);
    }
}

fn build_stream<T, O>(device: Device, config: StreamConfig, audio_buffer: Vec<T>, cursor: Arc<AtomicUsize>) -> Stream
    where
        T: Sample + Send + 'static,
        O: SizedSample + FromSample<T> {

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    
    let write_output = move |data: &mut [O], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            let index = cursor.fetch_add(1, Ordering::Relaxed); //I believe this could overflow after around 25 hours on 32 bit systems at 48 kHz
            if index < audio_buffer.len() {
                *sample = audio_buffer[index].to_sample::<O>();
            }
            else {
                *sample = Sample::EQUILIBRIUM;
            }
        }
    };

    device.build_output_stream(&config, write_output, err_fn, None)
        .expect("Should be able to build stream")
}
