#![feature(test)]

extern crate rodio;
extern crate test;

use test::Bencher;

use std::time::Duration;

use rodio::Source;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

/// always enable all features (except for repeat, for which this would not make sense)
fn full_audio(repeat: bool, pitch: f32, fade_in: Duration) -> BoxedSource<f32> {
    if repeat {
        let source = rodio::source::SineWave::new(440)
            .take_duration(Duration::from_secs(1))
            .repeat_infinite()
            .speed(pitch)
            .fade_in(fade_in)
            ;
        BoxedSource::new(source)
    } else {
        let source = rodio::source::SineWave::new(440)
            .take_duration(Duration::from_secs(1))
            .speed(pitch)
            .fade_in(fade_in);
        BoxedSource::new(source)
    }
}

/// dynamically compose features
fn dynamic_audio(repeat: bool, pitch: f32, fade_in: Duration) -> BoxedSource<f32> {
    let mut source = BoxedSource::new(
        rodio::source::SineWave::new(12000)
            .take_duration(Duration::from_secs(1))
            //.fade_in(Duration::from_millis(0))  // weirdly, this seems to make it faster
    );

    if repeat {
        source = BoxedSource::new(source.repeat_infinite());
    }

    if (pitch - 1.0).abs() < 1e-6 {
        source = BoxedSource::new(source.speed(pitch));
    }

    if fade_in > Duration::from_secs(0) {
        source = BoxedSource::new(source.fade_in(fade_in));
    }

    source
}

/// statically compose features
fn static_audio(repeat: bool, pitch: f32, fade_in: Duration) -> BoxedSource<f32> {
    let source =
        rodio::source::SineWave::new(12000)
            .take_duration(Duration::from_secs(1))
            //.fade_in(Duration::from_millis(0))  // weirdly, this seems to make it faster
            ;

    if (pitch - 1.0).abs() < 1e-6 {
        let source = source.speed(pitch);

        if repeat {
            let source = source.repeat_infinite();

            if fade_in > Duration::from_secs(0) {
                let source = source.fade_in(fade_in);
                BoxedSource::new(source)
            } else {
                BoxedSource::new(source)
            }
        } else {
            if fade_in > Duration::from_secs(0) {
                let source = source.fade_in(fade_in);
                BoxedSource::new(source)
            } else {
                BoxedSource::new(source)
            }
        }
    } else {
        if repeat {
            let source = source.repeat_infinite();

            if fade_in > Duration::from_secs(0) {
                let source = source.fade_in(fade_in);
                BoxedSource::new(source)
            } else {
                BoxedSource::new(source)
            }
        } else {
            if fade_in > Duration::from_secs(0) {
                let source = source.fade_in(fade_in);
                BoxedSource::new(source)
            } else {
                BoxedSource::new(source)
            }
        }
    }
}

#[bench]
fn bench_full_0(b: &mut Bencher) {
    b.iter(|| -> f32 {
        let source = full_audio(false, 1.0, Duration::from_millis(0));
        source.take(44800).sum()
    });
}

#[bench]
fn bench_full_1(b: &mut Bencher) {
    b.iter(|| -> f32 {
        let source = full_audio(true, 2.0, Duration::from_millis(500));
        source.take(44800).sum()
    });
}

#[bench]
fn bench_dynamic_0(b: &mut Bencher) {
    b.iter(|| -> f32 {
        let source = dynamic_audio(false, 1.0, Duration::from_millis(0));
        source.take(44800).sum()
    });
}

#[bench]
fn bench_dynamic_1(b: &mut Bencher) {
    b.iter(|| -> f32 {
        let source = dynamic_audio(true, 2.0, Duration::from_millis(500));
        source.take(44800).sum()
    });
}

#[bench]
fn bench_static_0(b: &mut Bencher) {
    b.iter(|| -> f32 {
        let source = static_audio(false, 1.0, Duration::from_millis(0));
        source.take(44800).sum()
    });
}

#[bench]
fn bench_static_1(b: &mut Bencher) {
    b.iter(|| -> f32 {
        let source = static_audio(true, 2.0, Duration::from_millis(500));
        source.take(44800).sum()
    });
}


/// Boxed wrapper over `rodio::Source` trait objects.
///
/// Used for dynamically composing sources.
struct BoxedSource<T> {
    input: Box<dyn rodio::Source<Item = T> + Send>,
}

impl<T> BoxedSource<T>
    where
        T: rodio::Sample,
{
    fn new<S: 'static>(source: S) -> Self
        where
            S: rodio::Source<Item = T> + Send,
    {
        BoxedSource {
            input: Box::new(source),
        }
    }
}

impl<T> rodio::Source for BoxedSource<T>
    where
        T: rodio::Sample,
{
    #[inline(always)]
    fn current_frame_len(&self) -> Option<usize> {
        self.input.current_frame_len()
    }

    #[inline(always)]
    fn channels(&self) -> u16 {
        self.input.channels()
    }

    #[inline(always)]
    fn sample_rate(&self) -> u32 {
        self.input.sample_rate()
    }

    #[inline(always)]
    fn total_duration(&self) -> Option<Duration> {
        self.input.total_duration()
    }
}

impl<T> Iterator for BoxedSource<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.input.next()
    }
}
