extern crate dsp4sdr;

use dsp4sdr::RealFs4Downconverter;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let n_samples = 1<<23 as usize;
    let n_repeats = 10u64;

    let dc = RealFs4Downconverter::new();
    let mut x: Vec<u16> = Vec::with_capacity(n_samples);
    for _ in 0..(n_samples/8) {
        x.push(2048+3); x.push(2048+7); x.push(2048+4); x.push(2048+6);
        x.push(2048-3); x.push(2048-7); x.push(2048-4); x.push(2048-6);
    }

    let now = std::time::Instant::now();
    for _ in 0..n_repeats {
        dc.process(&x);
    }
    let elapsed_time = now.elapsed();
    let total_samples = n_samples as f64 * n_repeats as f64;
    let total_time = elapsed_time.as_nanos() as f64 / 1e9;
    let throughput = total_samples / total_time;
    println!("{} blocks of {} samples, {:.2}Msps",
             n_repeats, n_samples, throughput / 1000000.0_f64);
}
