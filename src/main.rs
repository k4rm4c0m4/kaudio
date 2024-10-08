mod configurators;

use cpal::{
    traits::{DeviceTrait, StreamTrait},
    FromSample, Sample, SizedSample,
};

use log::{
    error,
    info,
};

use crate::configurators::clmenu;
use crate::configurators::logger;

fn main() -> anyhow::Result<()> {
    if let Err(e) = logger::configure(log::LevelFilter::Info) {
        error!("Failed to configure logger: {}", e);
        return Err(e);
    }

    let (config, device, frequency) = match clmenu::configure() {
        Ok((config, device, frequency)) => (config, device, frequency),
        Err(e) => {
            error!("Failed to configure audio: {}", e);
            return Err(e);
        }
    };

    info!("KAudio is running: {:?}", config);

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into(), frequency),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), frequency),
        // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into(), frequency),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into(), frequency),
        // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into(), frequency),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into(), frequency),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into(), frequency),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), frequency),
        // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into(), frequency),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into(), frequency),
        // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into(), frequency),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into(), frequency),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), frequency),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into(), frequency),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig, frequency: f32) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * frequency * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| error!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
