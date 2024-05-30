//! https://github.com/arkworks-rs/algebra/pull/804

use criterion::{criterion_group, criterion_main, Criterion};
use rayon::prelude::*;
use utils::run_bench;

use ark_ec::models::CurveConfig;
use ark_ed_on_bls12_381_bandersnatch::{BandersnatchConfig, EdwardsAffine, Fq, SWAffine};
use ark_ff::{Field, MontFp, One};
use ark_std::{rand::Rng, UniformRand};

// Constants used in mapping TE form to SW form and vice versa
//
// sage: q = 52435875175126190479447740508185965837690552500527637822603658699938581184513
// sage: Fq = GF(q)
// sage: MONT_A = 29978822694968839326280996386011761570173833766074948509196803838190355340952
// sage: MONT_B = 25465760566081946422412445027709227188579564747101592991722834452325077642517
// sage: MONT_A_OVER_THREE = MONT_A/Fq(3)
// 9992940898322946442093665462003920523391277922024982836398934612730118446984
// sage: MONT_B_INV = Fq(1)/MONT_B
// 41180284393978236561320365279764246793818536543197771097409483252169927600582
// sage: MONT_A_OVER_3B = MONT_A/(Fq(3)*MONT_B)
const MONT_A_OVER_THREE: Fq =
    MontFp!("9992940898322946442093665462003920523391277922024982836398934612730118446984");
const MONT_B_INV: Fq =
    MontFp!("41180284393978236561320365279764246793818536543197771097409483252169927600582");
const MONT_A_OVER_3B: Fq =
    MontFp!("42460977304182762931716743824405123254375045638571669698531889431804823178961");

/// Map an point in TE form to its corresponding  point on SW curve
pub fn map_te_to_sw(point: &EdwardsAffine) -> Option<SWAffine> {
    //First we map the point from the TE to Montgamory
    //edward to mont  ((1+y)/(1-y), (1+y)/(x(1-y)))
    let v_denom = <<BandersnatchConfig as CurveConfig>::BaseField as One>::one() - point.y;
    let w_denom = point.x - point.x * point.y;

    let v_denom_inv = v_denom.inverse()?;
    let w_denom_inv = w_denom.inverse()?;

    let v_w_num = <<BandersnatchConfig as CurveConfig>::BaseField as One>::one() + point.y;

    let v = v_w_num * v_denom_inv;
    let w = v_w_num * w_denom_inv;

    //now  we are mappyng the montgamory point to SW.
    //Mont->SW ((x+A/3)/B,y/B)

    let x = MONT_B_INV * v + MONT_A_OVER_THREE;
    let y = MONT_B_INV * w;

    let point_on_sw_curve = SWAffine::new_unchecked(x, y);
    debug_assert!(
        point_on_sw_curve.is_on_curve(),
        "TE point mapped off the SW curve"
    );

    return Some(point_on_sw_curve);
}

/// Map an point in TE form to its corresponding  point on SW curve
pub fn map_te_to_sw_opt(point: &EdwardsAffine) -> Option<SWAffine> {
    let one_plus_y = <<BandersnatchConfig as CurveConfig>::BaseField as One>::one() + point.y;
    let one_mins_y = <<BandersnatchConfig as CurveConfig>::BaseField as One>::one() - point.y;
    // let one_mins_y_inv = one_mins_y.inverse()?;
    let x_inv = point.x.inverse()?;

    let z = one_plus_y * one_mins_y.inverse()?;
    let z = z * MONT_B_INV;

    let x = z + MONT_A_OVER_3B;
    let y = z * x_inv;

    let point_on_sw_curve = SWAffine::new_unchecked(x, y);
    debug_assert!(
        point_on_sw_curve.is_on_curve(),
        "TE point mapped off the SW curve"
    );

    return Some(point_on_sw_curve);
}

pub fn random_vec<X: UniformRand, R: Rng>(n: usize, rng: &mut R) -> Vec<X> {
    (0..n).map(|_| X::rand(rng)).collect()
}

pub fn te_to_sw(count: usize) -> impl Fn() {
    let rng = &mut ark_std::test_rng();
    let pts = random_vec(count, rng);
    move || {
        let _ = pts.iter().map(|p| map_te_to_sw(p)).collect::<Vec<_>>();
    }
}

pub fn te_to_sw_par(count: usize) -> impl Fn() {
    let rng = &mut ark_std::test_rng();
    let pts = random_vec(count, rng);
    move || {
        let _ = pts.par_iter().map(|p| map_te_to_sw(p)).collect::<Vec<_>>();
    }
}

pub fn te_to_sw_opt(count: usize) -> impl Fn() {
    let rng = &mut ark_std::test_rng();
    let pts = random_vec(count, rng);
    move || {
        let _ = pts.iter().map(|p| map_te_to_sw_opt(p)).collect::<Vec<_>>();
    }
}

pub fn te_to_sw_opt_par(count: usize) -> impl Fn() {
    let rng = &mut ark_std::test_rng();
    let pts = random_vec(count, rng);
    move || {
        let _ = pts
            .par_iter()
            .map(|p| map_te_to_sw_opt(p))
            .collect::<Vec<_>>();
    }
}

fn te_sw_map(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("ed-te-map");
        run_bench("tw-to-sw-2048", &mut group, te_to_sw(2048));
        run_bench("tw-to-sw-opt-2048", &mut group, te_to_sw_opt(2048));
        run_bench("tw-to-sw-opt-par-2048", &mut group, te_to_sw_opt_par(2048));
    }
}

criterion_group!(benches, te_sw_map);
criterion_main!(benches);
