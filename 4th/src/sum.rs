#![allow(dead_code)]

pub fn sum(arr: &[u32]) -> Result<u32, String> {
    let s: u64 = arr.iter().map(|e| e.to_owned()).map(|e| e as u64).sum();
    if s > u32::MAX as u64 {
        return Err("over max u32".to_string());
    }
    return Ok(s as u32);
}
