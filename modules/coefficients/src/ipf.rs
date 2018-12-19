//! Definition of IPF Points.

use opltypes::*;

use std::f64::consts::SQRT_2;

/// Table of error function values case A, taken from DERF.C, referenced below.
#[rustfmt::skip]
const ERF_A: [f64; 65] = [
    5.958930743e-11, -1.13739022964e-9,
    1.466005199839e-8, -1.635035446196e-7,
    1.6461004480962e-6, -1.492559551950604e-5,
    1.2055331122299265e-4, -8.548326981129666e-4,
    0.00522397762482322257, -0.0268661706450773342,
    0.11283791670954881569, -0.37612638903183748117,
    1.12837916709551257377,
    2.372510631e-11, -4.5493253732e-10,
    5.90362766598e-9, -6.642090827576e-8,
    6.7595634268133e-7, -6.21188515924e-6,
    5.10388300970969e-5, -3.7015410692956173e-4,
    0.00233307631218880978, -0.0125498847718219221,
    0.05657061146827041994, -0.2137966477645600658,
    0.84270079294971486929,
    9.49905026e-12, -1.8310229805e-10,
    2.39463074e-9, -2.721444369609e-8,
    2.8045522331686e-7, -2.61830022482897e-6,
    2.195455056768781e-5, -1.6358986921372656e-4,
    0.00107052153564110318, -0.00608284718113590151,
    0.02986978465246258244, -0.13055593046562267625,
    0.67493323603965504676,
    3.82722073e-12, -7.421598602e-11,
    9.793057408e-10, -1.126008898854e-8,
    1.1775134830784e-7, -1.1199275838265e-6,
    9.62023443095201e-6, -7.404402135070773e-5,
    5.0689993654144881e-4, -0.00307553051439272889,
    0.01668977892553165586, -0.08548534594781312114,
    0.56909076642393639985,
    1.55296588e-12, -3.032205868e-11,
    4.0424830707e-10, -4.71135111493e-9,
    5.011915876293e-8, -4.8722516178974e-7,
    4.30683284629395e-6, -3.445026145385764e-5,
    2.4879276133931664e-4, -0.00162940941748079288,
    0.00988786373932350462, -0.05962426839442303805,
    0.49766113250947636708
];

/// Table of error function values case B, taken from DERF.C, referenced below.
#[rustfmt::skip]
const ERF_B: [f64; 65] = [
    -2.9734388465e-10, 2.69776334046e-9,
    -6.40788827665e-9, -1.6678201321e-8,
    -2.1854388148686e-7, 2.66246030457984e-6,
    1.612722157047886e-5, -2.5616361025506629e-4,
    1.5380842432375365e-4, 0.00815533022524927908,
    -0.01402283663896319337, -0.19746892495383021487,
    0.71511720328842845913,
    -1.951073787e-11, -3.2302692214e-10,
    5.22461866919e-9, 3.42940918551e-9,
    -3.5772874310272e-7, 1.9999935792654e-7,
    2.687044575042908e-5, -1.1843240273775776e-4,
    -8.0991728956032271e-4, 0.00661062970502241174,
    0.00909530922354827295, -0.2016007277849101314,
    0.51169696718727644908,
    3.147682272e-11, -4.8465972408e-10,
    6.3675740242e-10, 3.377623323271e-8,
    -1.5451139637086e-7, -2.03340624738438e-6,
    1.947204525295057e-5, 2.854147231653228e-5,
    -0.00101565063152200272, 0.00271187003520095655,
    0.02328095035422810727, -0.16725021123116877197,
    0.32490054966649436974,
    2.31936337e-11, -6.303206648e-11,
    -2.64888267434e-9, 2.050708040581e-8,
    1.1371857327578e-7, -2.11211337219663e-6,
    3.68797328322935e-6, 9.823686253424796e-5,
    -6.5860243990455368e-4, -7.5285814895230877e-4,
    0.02585434424202960464, -0.11637092784486193258,
    0.18267336775296612024,
    -3.67789363e-12, 2.0876046746e-10,
    -1.93319027226e-9, -4.35953392472e-9,
    1.8006992266137e-7, -7.8441223763969e-7,
    -6.75407647949153e-6, 8.428418334440096e-5,
    -1.7604388937031815e-4, -0.0023972961143507161,
    0.0206412902387602297, -0.06905562880005864105,
    0.09084526782065478489
];

/// Gauss error function.
///
/// This implementation is taken from the DERF.C implementation referenced below,
/// Copyright(C) 1996 Takuya Ooura (ooura@mmm.t.u-tokyo.ac.jp), released
/// under a free license (MIT-compatible, no attribution).
///
/// It was translated slightly to better fit Rust.
///
/// The implementation is intended to use most of the double-precision bit range,
/// which is more than sufficiently accurate for purposes of coefficients.
///
/// # References
///
/// https://en.wikipedia.org/wiki/Error_function
/// https://web.archive.org/web/20060925113650/http://momonga.t.u-tokyo.ac.jp/~ooura/gamerf.html
/// https://web.archive.org/web/20181104012221/https://www.jstatsoft.org/article/view/v011i04/v11i04.pdf
#[rustfmt::skip]
fn erf(x: f64) -> f64 {
    let mut k: i32;
    let mut t: f64;
    let mut y: f64;
    let w: f64 = x.abs();

    if w < 2.2 {
        t = w * w;
        k = t as i32;
        t -= f64::from(k);
        k *= 13;
        let z = k as usize;
        y = ((((((((((((ERF_A[z] * t + ERF_A[z + 1]) * t +
            ERF_A[z + 2]) * t + ERF_A[z + 3]) * t + ERF_A[z + 4]) * t +
            ERF_A[z + 5]) * t + ERF_A[z + 6]) * t + ERF_A[z + 7]) * t +
            ERF_A[z + 8]) * t + ERF_A[z + 9]) * t + ERF_A[z + 10]) * t +
            ERF_A[z + 11]) * t + ERF_A[z + 12]) * w;
    } else if w < 6.9 {
        k = w as i32;
        t = w - f64::from(k);
        k = 13 * (k - 2);
        let z = k as usize;
        y = (((((((((((ERF_B[z] * t + ERF_B[z + 1]) * t +
            ERF_B[z + 2]) * t + ERF_B[z + 3]) * t + ERF_B[z + 4]) * t +
            ERF_B[z + 5]) * t + ERF_B[z + 6]) * t + ERF_B[z + 7]) * t +
            ERF_B[z + 8]) * t + ERF_B[z + 9]) * t + ERF_B[z + 10]) * t +
            ERF_B[z + 11]) * t + ERF_B[z + 12];
        y *= y;
        y *= y;
        y *= y;
        y = 1.0 - y * y;
    } else {
        y = 1.0;
    }

    x.signum() * y
}

/// The cumulative distribution function of a generic normal distribution.
///
/// This represents the area of the graph under the given normal distribution
/// from negative infinity until `x`.
///
/// The returned value is the probability that the value a normally-distributed
/// random value (representing weight lifted) will be less than or equal to `x`.
///
/// A closed form solution does not exist for this integral.
///
/// https://en.wikipedia.org/wiki/Normal_distribution#Cumulative_distribution_function.
fn cumulative_normal(x: f64, mean: f64, deviation: f64) -> f64 {
    0.5 * (1.0 + erf((x - mean) / (deviation * SQRT_2)))
}

/// Hardcoded formula parameters: `(mean_1, mean_2, deviation_1, deviation_2)`.
type Parameters = (f64, f64, f64, f64);

/// Gets formula parameters from what is effectively a lookup table.
fn get_parameters(sex: Sex, equipment: Equipment, event: Event) -> Parameters {
    // Since the formula was made for the IPF, it only covers Raw and Single-ply.
    // We do our best and just reuse those for Wraps and Multi-ply, respectively.
    let equipment = match equipment {
        Equipment::Raw | Equipment::Wraps | Equipment::Straps => Equipment::Raw,
        Equipment::Single | Equipment::Multi => Equipment::Single,
    };

    // Full-power parameters.
    if event.is_full_power() {
        match (sex, equipment) {
            (Sex::M, Equipment::Raw) => (310.67, 857.785, 53.21602246, 147.0835213),
            (Sex::M, Equipment::Single) => (387.265, 1121.28, 80.63242593, 222.4895974),
            (Sex::F, Equipment::Raw) => (125.1435, 228.03, 34.52461523, 86.83009976),
            (Sex::F, Equipment::Single) => (176.58, 373.315, 48.45340151, 110.0102772),
            _ => (0.0, 0.0, 0.0, 0.0),
        }
    } else if event.is_squat_only() {
        match (sex, equipment) {
            (Sex::M, Equipment::Raw) => (123.1, 363.085, 25.16669173, 75.43114253),
            (Sex::M, Equipment::Single) => (150.485, 446.445, 36.51551612, 103.7060711),
            (Sex::F, Equipment::Raw) => (50.479, 105.632, 19.18458916, 56.22148694),
            (Sex::F, Equipment::Single) => (74.6855, 171.585, 21.94753597, 52.2948313),
            _ => (0.0, 0.0, 0.0, 0.0),
        }
    } else if event.is_bench_only() {
        match (sex, equipment) {
            (Sex::M, Equipment::Raw) => (86.4745, 259.155, 17.57845791, 53.12202336),
            (Sex::M, Equipment::Single) => (133.94, 441.465, 35.39379355, 113.0057151),
            (Sex::F, Equipment::Raw) => (25.0485, 43.848, 6.717175515, 13.95197273),
            (Sex::F, Equipment::Single) => (49.106, 124.209, 23.19897729, 67.4926054),
            _ => (0.0, 0.0, 0.0, 0.0),
        }
    } else if event.is_deadlift_only() {
        match (sex, equipment) {
            (Sex::M, Equipment::Raw) => (103.5355, 244.765, 15.3713591, 31.50223091),
            (Sex::M, Equipment::Single) => (110.135, 263.66, 14.99598937, 23.01097909),
            (Sex::F, Equipment::Raw) => (47.136, 67.349, 9.155512107, 13.66997543),
            (Sex::F, Equipment::Single) => (51.002, 69.8265, 8.58023763, 5.725798366),
            _ => (0.0, 0.0, 0.0, 0.0),
        }
    } else {
        (0.0, 0.0, 0.0, 0.0)
    }
}

/// Performs the points calculation without applying a scaling factor.
fn percentile(
    sex: Sex,
    equipment: Equipment,
    event: Event,
    bodyweight: WeightKg,
    total: WeightKg,
) -> f64 {
    // Look up parameters.
    let (mean1, mean2, dev1, dev2) = get_parameters(sex, equipment, event);

    // Calculate the properties of the normal distribution.
    let bw_log = f64::from(bodyweight).ln();
    let mean = mean1 * bw_log - mean2;
    let dev = dev1 * bw_log - dev2;

    cumulative_normal(f64::from(total), mean, dev)
}

/// Calculates IPF Points.
pub fn ipf(
    sex: Sex,
    equipment: Equipment,
    event: Event,
    bodyweight: WeightKg,
    total: WeightKg,
) -> Points {
    // FIXME: This is a temporary value.
    // FIXME: Apparently the IPF will have an official scaling function.
    const SCALING_FACTOR: f64 = 1000.0;
    Points::from(percentile(sex, equipment, event, bodyweight, total) * SCALING_FACTOR)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cumulative_normal_accuracy() {
        // Check a few cases of a standard normal.
        assert_eq!(cumulative_normal(1.0, 0.0, 1.0), 0.8413447460685429);
        assert_eq!(cumulative_normal(0.0, 0.0, 1.0), 0.5);
    }

    #[test]
    fn check_canonical_percentiles() {
        // Daniella Melo.
        // FIXME: The actual value was just 0.999986, but we can't do epsilon
        // FIXME: comparisons yet. We should really get that ability.
        assert_eq!(
            percentile(
                Sex::F,
                Equipment::Raw,
                Event::sbd(),
                WeightKg::from_f32(83.5),
                WeightKg::from_f32(601.5)
            ),
            0.9999855952672011
        );
    }
}
