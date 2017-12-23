//! Univariate analysis

mod bootstrap;
mod percentiles;
mod resamples;
mod sample;

pub mod kde;
pub mod mixed;
pub mod outliers;

use ::float::Float;
use num_cpus;
use thread_scoped as thread;
use std::cmp;

use tuple::{Tuple, TupledDistributions, TupledDistributionsBuilder};

use self::resamples::Resamples;

pub use self::percentiles::Percentiles;
pub use self::sample::Sample;

/// Performs a two-sample bootstrap
///
/// - Multithreaded
/// - Time: `O(nresamples)`
/// - Memory: `O(nresamples)`
#[cfg_attr(clippy, allow(cast_lossless))]
pub fn bootstrap<A, B, T, S>(
    a: &Sample<A>,
    b: &Sample<B>,
    nresamples: usize,
    statistic: S,
) -> T::Distributions where
    A: Float,
    B: Float,
    S: Fn(&Sample<A>, &Sample<B>) -> T,
    S: Sync,
    T: Tuple,
    T::Distributions: Send,
    T::Builder: Send,
{
    let ncpus = num_cpus::get();

    unsafe {
        // TODO need some sensible threshold to trigger the multi-threaded path
        if ncpus > 1 && nresamples > a.as_slice().len() + b.as_slice().len() {
            let granularity = nresamples / ncpus + 1;
            let granularity_sqrt = (granularity as f64).sqrt().ceil() as usize;
            let ref statistic = statistic;

            let chunks = (0..ncpus).map(|i| {
                let mut sub_distributions: T::Builder =
                        TupledDistributionsBuilder::new(granularity);
                let offset = i * granularity;

                thread::scoped(move || {
                    let end = cmp::min(offset + granularity, nresamples);
                    let mut a_resamples = Resamples::new(a);
                    let mut b_resamples = Resamples::new(b);
                    let mut i = offset;

                    for _ in 0..granularity_sqrt {
                        let a_resample = a_resamples.next();

                        for _ in 0..granularity_sqrt {
                            if i == end {
                                return sub_distributions;
                            }

                            let b_resample = b_resamples.next();

                            sub_distributions.push(statistic(a_resample, b_resample));

                            i += 1;
                        }
                    }
                    sub_distributions
                })
            }).collect::<Vec<_>>();

            let mut builder: T::Builder =
                    TupledDistributionsBuilder::new(nresamples);
            for mut chunk in chunks {
                builder.extend(&mut (chunk.join()));
            }
            builder.complete()
        } else {
            let nresamples_sqrt = (nresamples as f64).sqrt().ceil() as usize;
            let mut a_resamples = Resamples::new(a);
            let mut b_resamples = Resamples::new(b);
            let mut distributions: T::Distributions =
                TupledDistributions::uninitialized(nresamples);

            let mut i = 0;
            'outer: for _ in 0..nresamples_sqrt {
                let a_resample = a_resamples.next();

                for _ in 0..nresamples_sqrt {
                    if i == nresamples {
                        break 'outer;
                    }

                    let b_resample = b_resamples.next();

                    distributions.set_unchecked(i, statistic(a_resample, b_resample));

                    i += 1;
                }
            }

            distributions
        }
    }
}
