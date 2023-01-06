/// If a number is between -EPSILON and EPSILON it is
/// considered 0.
pub const EPSILON: f32 = 0.0001;
/// The number of iterations used in false position method,
/// this doesn't need to be a big number as floats have a
/// maximum precision.
pub const ITERATIONS: i32 = 20;
/// The size of the partitions of the roots bounds.
/// If there is 2 roots in the same partition only 1 is
/// going to be found.
/// This doesn't means that the minimum distance of the
/// roots is PARTITION_SIZE, just that 2 roots must be in
/// different partitions.
pub const PARTITION_SIZE: f32 = 0.01;
