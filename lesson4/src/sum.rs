
const MAX_U32: u32 = 0xFFFFFFFF;

pub fn get_sum(array: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for num in array {
        if MAX_U32 - sum < *num {
            // sum is overflow
            return None;
        }
        sum = sum + num;
    }

    Some(sum)
}