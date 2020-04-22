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

// [http://mijkenator.github.io/2016/12/10/2016-12-10-mergesort-inversion-count/]
//
fn merge_count_inversion(arr: Vec<u16>) -> (Vec<u16>, usize) {
	if arr.len() <= 1 {
		return (arr, 0);
	}
	let middle = arr.len() / 2;
	let (left, a) = merge_count_inversion(arr[..middle].to_vec());
	let (right, b) = merge_count_inversion(arr[middle..].to_vec());
	let (result, c) = merge_count_split_inversion(left, right);
	(result, a + b + c)
}

fn get_blank_index(arr: &Vec<u16>) -> usize {
	for i in 0..arr.len() {
		if arr[i] == 0 {
			return i;
		}
	}
	panic!("Why there is no blank ?!");
}

fn get_inversions_naive(arr: &Vec<u16>) -> usize {
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

// [https://www.geeksforgeeks.org/counting-inversions/]
//
fn get_inversion_count_with_merge_sort(arr: &Vec<u16>) -> usize {
	let mut tmp = arr.clone();
	tmp.remove(get_blank_index(&tmp));
	let inversions = merge_count_inversion(tmp).1;
	inversions
}

#[inline]
fn get_blank_row_from_bottom(mflat: &Vec<u16>, msize: usize) -> usize {
	msize - get_blank_index(mflat) / msize
}

#[inline]
fn is_even(n: usize) -> bool {
	n % 2 == 0
}

#[inline]
fn get_inversions(arr: &Vec<u16>) -> usize {
	get_inversion_count_with_merge_sort(arr)
}

// [https://www.geeksforgeeks.org/check-instance-15-puzzle-solvable/]
// [https://www.cs.bham.ac.uk/~mdr/teaching/modules04/java2/TilesSolvability.html]
//
// Formula for determining solvability:
// ((grid width is odd) && (# is even)) || ((grid width is even) && ((blank is on odd row from bottom) == (# is even)))
//
fn is_solvable(mflat: &Vec<u16>, msize: usize) -> bool {
	let inversions = get_inversions(mflat);
	if !is_even(msize) {
		is_even(inversions)
	} else {
		let blank_row = get_blank_row_from_bottom(mflat, msize);
		is_even(inversions) == !is_even(blank_row)
	}
}

fn is_format_coherent(matrix: &Vec<Vec<u16>>, msize: usize) -> bool {
	matrix.len() == msize && matrix.iter().all(|row| row.len() == msize)
}

pub fn check_puzzle(matrix: &Vec<Vec<u16>>, msize: usize) -> bool {
	let flatten_matrix: Vec<u16> = matrix.iter().flat_map(|row| row.iter()).cloned().collect();
	is_format_coherent(matrix, msize) && is_solvable(&flatten_matrix, msize)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]

	fn test_inversions() {
		let vec1 = vec![3, 1, 2];
		let vec2 = vec![8, 4, 2, 1];
		let vec3 = vec![1, 2, 3, 5, 4, 6];
		let vec4 = vec![6, 4, 5, 1, 3, 2];
		let vec5 = vec![1, 2, 4, 3, 5, 6, 7, 9, 8];
		let vec6 = vec![2, 1, 3, 5, 4];
		let vec7 = vec![1, 9, 6, 4, 5];
		let vec8 = vec![12, 1, 10, 2, 7, 11, 4, 14, 5, 9, 15, 8, 13, 6, 3];
		assert_eq!(get_inversions(&vec1), get_inversions_naive(&vec1));
		assert_eq!(get_inversions(&vec2), get_inversions_naive(&vec2));
		assert_eq!(get_inversions(&vec3), get_inversions_naive(&vec3));
		assert_eq!(get_inversions(&vec4), get_inversions_naive(&vec4));
		assert_eq!(get_inversions(&vec5), get_inversions_naive(&vec5));
		assert_eq!(get_inversions(&vec6), get_inversions_naive(&vec6));
		assert_eq!(get_inversions(&vec7), get_inversions_naive(&vec7));
		assert_eq!(get_inversions(&vec8), get_inversions_naive(&vec8));
	}

	#[test]
	fn test_get_inversions_with_blank() {
		let vec1 = vec![12, 1, 10, 2, 7, 11, 4, 14, 5, 0, 9, 15, 8, 13, 6, 3];
		let vec2 = vec![12, 1, 10, 2, 7, 0, 4, 14, 5, 11, 9, 15, 8, 13, 6, 3];
		let vec3 = vec![7, 1, 2, 5, 3, 9, 8, 0, 6];
		let vec4 = vec![7, 1, 2, 5, 0, 9, 8, 3, 6];
		let mut arr1 = vec1.clone();
		let mut arr2 = vec2.clone();
		let mut arr3 = vec3.clone();
		let mut arr4 = vec4.clone();
		arr1.remove(get_blank_index(&arr1));
		arr2.remove(get_blank_index(&arr2));
		arr3.remove(get_blank_index(&arr3));
		arr4.remove(get_blank_index(&arr4));
		assert_eq!(get_inversions_naive(&arr1), 49);
		assert_eq!(get_inversions_naive(&arr2), 48);
		assert_eq!(get_inversions_naive(&arr3), 9);
		assert_eq!(get_inversions_naive(&arr4), 11);
		assert_eq!(get_inversions(&arr1), get_inversions_naive(&arr1));
		assert_eq!(get_inversions(&arr2), 48);
		assert_eq!(get_inversions(&arr3), 9);
		assert_eq!(get_inversions(&arr4), 11);
	}

	// #[test]
	// fn test_blank_row() {
	// 	assert_eq!(
	// 		get_blank_row_from_bottom(
	// 			&vec![12, 1, 10, 2, 7, 11, 4, 14, 5, 0, 9, 15, 8, 13, 6, 3],
	// 			4
	// 		),
	// 		2
	// 	);
	// 	assert_eq!(
	// 		get_blank_row_from_bottom(
	// 			&vec![12, 1, 10, 2, 7, 0, 4, 14, 5, 11, 9, 15, 8, 13, 6, 3],
	// 			4
	// 		),
	// 		3
	// 	);
	// 	assert_eq!(
	// 		get_blank_row_from_bottom(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8], 3),
	// 		3
	// 	);
	// 	assert_eq!(
	// 		get_blank_row_from_bottom(
	// 			&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
	// 			4
	// 		),
	// 		4
	// 	);
	// }

	// #[test]
	// fn test_check_puzzle() {
	// 	assert_eq!(
	// 		check_puzzle(
	// 			&vec![
	// 				vec![1, 2, 3, 4],
	// 				vec![5, 6, 7, 8],
	// 				vec![9, 10, 11, 12],
	// 				vec![13, 14, 15, 0]
	// 			],
	// 			4
	// 		),
	// 		true
	// 	);
	// 	assert_eq!(
	// 		check_puzzle(
	// 			&vec![
	// 				vec![1, 2, 3, 4],
	// 				vec![5, 6, 7, 8],
	// 				vec![9, 10, 11, 12],
	// 				vec![13, 15, 14, 0]
	// 			],
	// 			4
	// 		),
	// 		false
	// 	);
	// }
}
