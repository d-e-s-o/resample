use std::cmp::PartialOrd;
use std::f64::consts::PI;

use rstest::rstest;

use crate::ConverterType;


#[rustfmt::skip]
#[rstest(n, from_rate, to_rate, n_ch, bleed_size, in_bleed_eps, out_bleed_eps,
    case(16384, 1, 2, 1, 512, 1e-6, 0.02),
    case(16384, 1, 2, 2, 512, 1e-6, 0.02),
    case(16384, 1, 2, 7, 512, 1e-6, 0.02),
    case(44100, 44100, 48000, 1, 512, 1e-6, 0.001),
    case(44100, 44100, 48000, 2, 512, 1e-6, 0.001),
    case(44100, 44100, 48000, 7, 512, 1e-6, 0.001),
    case(22050, 44100, 16000, 1, 512, 1e-6, 0.02),
    case(22050, 44100, 16000, 2, 512, 1e-6, 0.02),
)]
fn simple_resample(
    n: usize,
    from_rate: usize,
    to_rate: usize,
    n_ch: u8,
    #[values(
        ConverterType::SincBestQuality,
        ConverterType::SincMediumQuality,
        ConverterType::SincFastest,
        ConverterType::ZeroOrderHold,
        ConverterType::Linear
    )]
    converter: ConverterType,
    bleed_size: usize,
    in_bleed_eps: f32,
    out_bleed_eps: f32,
) {
    let n_chs = usize::from(n_ch);
    let sig_freq = 128.0f64;
    let data = (0..n * n_chs)
        .map(|i| (2.0 * PI * sig_freq * ((i / n_chs) as f64 / n as f64)).sin())
        .map(|x| x as f32)
        .collect::<Vec<f32>>();

    let down_data = crate::convert(
        converter,
        n_ch,
        from_rate as u32,
        to_rate as u32,
        &data,
    )
    .unwrap();

    let up_data = crate::convert(
        converter,
        n_ch,
        to_rate as u32,
        from_rate as u32,
        &down_data,
    )
    .unwrap();

    // For now we only assert differences for the best quality
    // resampling mode.
    if converter == ConverterType::SincBestQuality {
        assert_eq!(
            up_data.len(),
            ((n * to_rate).div_ceil(from_rate) * from_rate).div_ceil(to_rate) * n_chs
        );

        let max_diff = data
            .iter()
            .enumerate()
            .zip(up_data.iter())
            .map(|((i, a), b)| (i, (a - b).abs()))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        let max_diff_bleed = data
            .iter()
            .take((n - bleed_size) * n_chs)
            .enumerate()
            .skip(bleed_size * n_chs)
            .zip(up_data.iter().skip(bleed_size * n_chs))
            .map(|((i, a), b)| (i, (a - b).abs()))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        assert!(max_diff_bleed.1 < in_bleed_eps);
        assert!(max_diff.1 < out_bleed_eps);
    } else {
      let expected = ((n * to_rate).div_ceil(from_rate) * from_rate).div_ceil(to_rate) * n_chs;
      let lower = expected - 3;
      let upper = expected + 7;

      assert!(up_data.len() >= lower, "{} | {lower}", up_data.len());
      assert!(up_data.len() <= upper, "{} | {upper}", up_data.len());
    }
}
