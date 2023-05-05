use super::SampleBuffer;

pub fn reverse(sample: &mut SampleBuffer) {
    for channel in sample.samples.iter_mut() {
        channel.reverse()
    }
}

pub fn invert(sample: &mut SampleBuffer) {
    for channel in sample.samples.iter_mut() {
        for sample in channel.iter_mut() {
            *sample *= -1.0;
        }
    }
}

// pub fn convolve(
//     input: &SampleBuffer, 
//     ir: &SampleBuffer
// ) -> SampleBuffer {
//     if input.sample_rate_original != ir.sample_rate_original {
//         todo!("non matching sample rate")
//     }
//     let ir = &ir.audio()[0];
//     let mut out: Vec<Vec<f32>> = Vec::with_capacity(input.channels());

//     for channel in input.audio() {
//         out .push(_convolve(&channel, ir))
//     }

//     SampleBuffer::new(out, input.sample_rate_original)
// }


pub fn mix(
    input: &SampleBuffer, 
    input_2: &SampleBuffer
) -> SampleBuffer {
    let mut out: Vec<Vec<f32>> = Vec::with_capacity(input.channels());

    for (smp_1_ch, smp_2_ch) in input.audio().iter().zip(input_2.audio().iter()) {
        let mut channel: Vec<f32> = Vec::with_capacity(smp_1_ch.len());

        for i in 0..smp_1_ch.len() {
            channel.push(smp_1_ch.get(i).unwrap_or(&0.0) + smp_2_ch.get(i).unwrap_or(&0.0));
        }
        out.push(channel);
    }
    SampleBuffer::new(out, input.sample_rate_original)
}

// /// :)
// fn _convolve(
//     input: &[f32],
//     ir: &[f32],
// ) -> Vec<f32> {
//     let len: usize = input.len() + ir.len() - 1;
//     let mut out = vec![0.0_f32; len];

//     for i in 0..out.len() {
//         for j in 0..ir.len() {
//             let idx = i as isize - j as isize;
//             if idx < 0 || idx >= input.len() as isize {
//                 continue;
//             }

//             out[i] += ir[j] * input[ i - j];
//         }
//     }

//     out
// }

fn _fft_convolve() {}


mod resampler {

}

