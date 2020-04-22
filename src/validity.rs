fn merge_sort(arr: &[u16], tmp: &mut Vec<u16>) -> usize {
	if arr.len() == 1 {
		return 0;
	}
	let mut inversions = 0;
	let mid = arr.len() / 2;
	let left = &arr[..mid];
	let right = &arr[mid..];
	inversions += merge_sort(left, tmp);
	inversions += merge_sort(right, tmp);
	let mut i = 0;
	let mut j = 0;
	while i < left.len() && j < right.len() {
		if left[i] <= right[j] {
			tmp.push(left[i]);
			i += 1;
		} else {
			tmp.push(right[j]);
			inversions += left.len() - i;
			j += 1;
		}
	}
	tmp.extend_from_slice(&left[i..]);
	tmp.extend_from_slice(&right[j..]);
	inversions
}

fn get_blank_index(arr: &Vec<u16>) -> usize {
	for i in 0..arr.len() {
		if arr[i] == 0 {
			return i;
		}
	}
	panic!("Why there is no blank ?!");
}

// [https://www.geeksforgeeks.org/counting-inversions/]
//
fn get_inversion_count_with_merge_sort(arr: &Vec<u16>) -> usize {
	let mut tmp = vec![];
	let inversions = merge_sort(arr, &mut tmp);
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
		assert_eq!(get_inversions(&vec![3, 1, 2]), 2);
		assert_eq!(get_inversions(&vec![8, 4, 2, 1]), 6);
	}

	#[test]
	fn test_get_inversions_with_blank() {
		assert_eq!(
			get_inversions(&vec![12, 1, 10, 2, 7, 11, 4, 14, 5, 0, 9, 15, 8, 13, 6, 3]),
			49
		);
		assert_eq!(
			get_inversions(&vec![12, 1, 10, 2, 7, 0, 4, 14, 5, 11, 9, 15, 8, 13, 6, 3]),
			48
		);
		assert_eq!(get_inversions(&vec![7, 1, 2, 5, 3, 9, 8, 0, 6]), 9);
		assert_eq!(get_inversions(&vec![7, 1, 2, 5, 0, 9, 8, 3, 6]), 11);
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
