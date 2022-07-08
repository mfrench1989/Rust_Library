pub fn avgMean(nums_in : &[f64]) -> f64 {
    if nums_in.is_empty() {
        return 0.0;
    }

    return nums_in.iter().sum::<f64>() / nums_in.len() as f64;
}

pub fn avgMedian(nums_in : &mut [f64]) -> f64 {
    if nums_in.is_empty() {
        return 0.0;
    }

    nums_in.sort_by(|a, b| a.partial_cmp(b).unwrap());
    return nums_in[nums_in.len() / 2];
}

pub fn avgMode(nums_in : &[f64]) -> f64 {
	if nums_in.is_empty() {
		return 0.0;
	}
	
	let mut num_count = HashMap::new();
	for &value in nums_in {
		*num_count.entry(value).or_insert(0) += 1;
	}

	return num_count
		.into_iter()
		.max_by_key(|&(_, count)| count)
		.map(|(val, _)| val)
		.expect("Cannot compute the mode of zero numbers")
}