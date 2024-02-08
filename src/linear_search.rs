fn linear_search<T: PartialEq>(arr: Vec<T>, target: T) -> Option<usize> {
    for i in 0..arr.len() {
        if arr[i] == target {
            return Some(i);
        }
    }
    None
}
enum OutOfBound {
    StartGreaterThanEnd,
    EndGreaterThanLength,
    StartLessThanZero,
}

fn search_in_range<T: PartialEq>(
    arr: Vec<T>,
    target: T,
    start: usize,
    end: usize,
) -> Result<Option<usize>, OutOfBound> {
    if start > end {
        return Err(OutOfBound::StartGreaterThanEnd);
    }
    if end >= arr.len() {
        return Err(OutOfBound::EndGreaterThanLength);
    }
    if start >= arr.len() {
        return Err(OutOfBound::StartLessThanZero);
    }
    for i in start..=end {
        if arr[i] == target {
            return Ok(Some(i));
        }
    }
    Ok(None)
}

fn find_min<T: Copy>(arr: Vec<T>) -> Option<T> {
    if arr.len() == 0 {
        return None;
    }
    let mut min = arr[0];

    for &i in arr.iter() {
        if i < min {
            min = i;
        }
    }
    Some(min)
}
