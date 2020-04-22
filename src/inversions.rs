fn merge_count_split_inversion(left: Vec<u16>, right: Vec<u16>) -> (Vec<u16>, usize) {
	let mut result = vec![];
	let mut count = 0;
	let mut i = 0;
	let mut j = 0;
	while i < left.len() && j < right.len() {
		if left[i] <= right[j] {
			result.push(left[i]);
			i += 1;
		} else {
			result.push(right[j]);
			count += left.len() - i;
			j += 1;
		}
	}
	result.extend_from_slice(&left[i..]);
	result.extend_from_slice(&right[j..]);
	(result, count)
}

fn merge_count_inversion(arr: &[u16]) -> (Vec<u16>, usize) {
	if arr.len() <= 1 {
		return (arr.to_vec(), 0);
	}
	let middle = arr.len() / 2;
	let (left, a) = merge_count_inversion(&arr[..middle]);
	let (right, b) = merge_count_inversion(&arr[middle..]);
	let (result, c) = merge_count_split_inversion(left, right);
	(result, a + b + c)
}

// [https://www.geeksforgeeks.org/counting-inversions/]
// [http://mijkenator.github.io/2016/12/10/2016-12-10-mergesort-inversion-count/]
//
pub fn merge_sort(arr: &Vec<u16>) -> usize {
	let (_, inversions) = merge_count_inversion(arr);
	inversions
}

pub fn naive(arr: &Vec<u16>) -> usize {
	let mut inversions = 0;
	for i in 0..arr.len() - 1 {
		for j in (i + 1)..arr.len() {
			if arr[i] > arr[j] {
				inversions += 1;
			}
		}
	}
	inversions
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_inversions_known() {
		let vec1 = vec![12, 1, 10, 2, 7, 11, 4, 14, 5, 9, 15, 8, 13, 6, 3];
		let vec2 = vec![12, 1, 10, 2, 7, 4, 14, 5, 11, 9, 15, 8, 13, 6, 3];
		let vec3 = vec![7, 1, 2, 5, 3, 9, 8, 6];
		let vec4 = vec![7, 1, 2, 5, 9, 8, 3, 6];
		assert_eq!(naive(&vec1), 49);
		assert_eq!(naive(&vec2), 48);
		assert_eq!(naive(&vec3), 9);
		assert_eq!(naive(&vec4), 11);
		assert_eq!(merge_sort(&vec1), 49);
		assert_eq!(merge_sort(&vec2), 48);
		assert_eq!(merge_sort(&vec3), 9);
		assert_eq!(merge_sort(&vec4), 11);
	}

	#[test]
	fn test_implem_match() {
		let vec1 = vec![3, 1, 2];
		let vec2 = vec![8, 4, 2, 1];
		let vec3 = vec![1, 2, 3, 5, 4, 6];
		let vec4 = vec![6, 4, 5, 1, 3, 2];
		let vec5 = vec![1, 2, 4, 3, 5, 6, 7, 9, 8];
		let vec6 = vec![2, 1, 3, 5, 4];
		let vec7 = vec![1, 9, 6, 4, 5];
		let vec8 = vec![12, 1, 10, 2, 7, 11, 4, 14, 5, 9, 15, 8, 13, 6, 3];
		assert_eq!(merge_sort(&vec1), naive(&vec1));
		assert_eq!(merge_sort(&vec2), naive(&vec2));
		assert_eq!(merge_sort(&vec3), naive(&vec3));
		assert_eq!(merge_sort(&vec4), naive(&vec4));
		assert_eq!(merge_sort(&vec5), naive(&vec5));
		assert_eq!(merge_sort(&vec6), naive(&vec6));
		assert_eq!(merge_sort(&vec7), naive(&vec7));
		assert_eq!(merge_sort(&vec8), naive(&vec8));
	}

	#[test]
	fn test_inversions_random() {
		for _ in 0..2000 {
			let mut vec: Vec<u16> = Vec::with_capacity(39);
			for _ in 0..vec.capacity() {
				vec.push(rand::random());
			}
			assert_eq!(merge_sort(&vec), naive(&vec));
		}
	}
}
