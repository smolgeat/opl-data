//! Auto-disambiguation for lifters with the same username.

use crate::{AllMeetData, LifterMap};


/// Represents the calculated (dis)similarity of an [Entry] pair.
///
/// Similarity is represented as a score in the range `[-100, 100]`.
/// Positive values express similarity, and negative values express dissimilarity.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Similarity(f64);

impl Similarity {
    pub const MAX: f64 = 100.0;
    pub const MIN: f64 = -100.0;
}

impl From<Distance> for Similarity {
    /// Linearly transforms a [Distance] range into a [Similarity] range.
    fn from(d: Distance) -> Similarity {
        const DST_RANGE: f64 = Distance::MAX - Distance::MIN;
        const SIM_RANGE: f64 = Similarity::MAX - Similarity::MIN;

        // Normalize the distance to [0,1].
        let normalized = (d.0 - Distance::MIN) / DST_RANGE;

        // Flip the range to [-1,0].
        let flipped = normalized * -1.0;

        // Convert into [Similarity::MIN, Similarity::MAX].
        Similarity(flipped * SIM_RANGE + Similarity::MAX)
    }
}


/// Represents the distance between an [Entry] pair in n-dimensional space.
///
/// Distance is calculated using the Chebyshev distance: each coordinate is considered
/// independently, and the distance is the maximum coordinate distance encountered.
///
/// Since the scale is arbitrary, we set the greatest possible distance to `1.0`.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Distance(f64);

impl Distance {
    pub const MAX: f64 = 1.0;
    pub const MIN: f64 = 0.0;

    /// Returns the maximum of two distances.
    pub fn max(self, other: Distance) -> Distance {
        Distance(self.0.max(other.0))
    }
}


/// Performs auto-disambiguation.
///
/// Consumes the original LifterMap, producing an updated LifterMap.
/// The AllMeetData entries are modified to match new disambiguations.
///
pub fn infer(liftermap: LifterMap, meetdata: &mut AllMeetData) -> LifterMap {
    // TODO: Skip entries if they're in the lifterdata, like for sex exemptions
    for username in liftermap.keys() {
        // Inference only checks un-disambiguated usernames: "janedoe", not "janedoe1".
        if username.ends_with(|c: char| c.is_digit(10)) {
            continue;
        }

        infer_for(username, &liftermap, meetdata);
    }

    // Generate a new LifterMap from the changed AllMeetData.
    meetdata.create_liftermap()
}


/// Performs auto-disambiguation for a single base username.
pub fn infer_for(username: &str, liftermap: &LifterMap, meetdata: &mut AllMeetData) {
    // Make a source for each variant.
    // Make a sink for each variant.
    //      Fill the manually-disambiguated sinks with the manually-disambiguated data.
    //
    // For each entry in the base username's source, sorted by date:
    //      Compare to each sink. If compatible, place in the earliest sink.
    //      If not compatible, go to the next sink.
    //      If not compatible with any sink, make a new one and place it there.
    //
    //      Really we need a function that we're trying to optimize.
    //      This needs to be expressed as a mathematical function.
    //
    //      Something like "degree of similarity".
    //
    //      Suppose we have some cmp(A,B) function that compares the similarity
    //      of two entries. So that's like == 1.0 if compatible, == -1.0 if completely
    //      incompatible, with some linear amount in between that.
    //
    //          It has to be -1.0, otherwise it would always be optimal to put all
    //          the entries in a single bucket, since every pair would be present.
    //
    //      Then we want to make N buckets such that
    //
    //          \sum_1^n { \forall (A,B \in bucket_i | A \ne B) cmp(A,B) }
    //
    //          is maximized.
    //
    //      So just to get some intution, suppose we have buckets (A,B) and (C,D).
    //      If we we move B to the other bucket, that affects the score like:
    //
    //          - cmp(A,B) + cmp(B,C) + cmp(B,D).
    //
    //      That's a good move if that's greater than zero, AKA:
    //
    //          => cmp(B,C) + cmp(B,D) - cmp(A,B) > 0
    //          => cmp(B,C) + cmp(B,D) > cmp(A,B)
    //
    //      The optimum is found in the space of all permutations with separators.
    //
    //      How do you know if you should make a new category? Well, you don't
    //      put it into any category where the sum of its cmp() functions with
    //      its neighbors is negative. So you move it to the most-positive category,
    //      and if they're all negative, you make a new one.
    //
    //      Well... except for we're not really interested in comparing with *all*
    //      the things in the bucket. If a lifter competed in 2020 and in 1970,
    //      we're not interested in the similarity of those two results. We're interested
    //      in the similarity of *nearby* results.
    //
    //      Also, the above formulation double-counts, since it does cmp(B,C) and cmp(C,B).
    //      We don't really care about that.
    //
    //      So, how about: for A_n, we compare against A_{n-1} and A_{n-2}.
    //      Wait, that's kind of stupid. Maybe the compatibility function should weight by date?
    //      So if you're far away, then it matters less. Then if we want that to matter *zero*,
    //      we can certainly do so.
    //
    //      We should probably use the Chebyshev distance: the trait that produces
    //      the *greatest* distance is the one that's used, still positive or negative
    //      (but the distance is taken as the absolute value).
    //          
    //
    // If there's more than 1 sink, go through and rename everything.
    // Newly-created sinks get named like #3A (for "Auto").
    //
    //      Compare pairwise 
    //
    // For each entry in the base username's bucket, sorted by date:
    //
    //      Look for the 

    // Are there manually-disambiguated variants?
    //
    // Sort them all by date.
    //
    // 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_to_similarity() {
        // Minimum Distance is maximum Similarity.
        let d = Distance(Distance::MIN);
        assert_eq!(Similarity::from(d), Similarity(Similarity::MAX));

        // Maximum Distance is minimum Similarity.
        let d = Distance(Distance::MAX);
        assert_eq!(Similarity::from(d), Similarity(Similarity::MIN));
    }
}
