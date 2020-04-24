use super::inversions;

fn get_blank_index(arr: &Vec<u16>) -> usize {
	arr.iter()
		.position(|&n| n == 0)
		.expect("No blank, invalid puzzle !")
}

fn get_blank_row_from_bottom(mflat: &Vec<u16>, msize: usize) -> usize {
	msize - get_blank_index(mflat) / msize
}

fn is_even(n: usize) -> bool {
	n % 2 == 0
}

// [https://www.geeksforgeeks.org/check-instance-15-puzzle-solvable/]
// [https://www.cs.bham.ac.uk/~mdr/teaching/modules04/java2/TilesSolvability.html]
//
// Formula for determining solvability:
// ((grid width is odd) && (# is even)) || ((grid width is even) && ((blank is on odd row from bottom) == (# is even)))
//
fn is_solvable(mflat: &Vec<u16>, msize: usize, inversions: usize) -> bool {
	if !is_even(msize) {
		is_even(inversions)
	} else {
		let blank_row = get_blank_row_from_bottom(mflat, msize);
		is_even(inversions) == !is_even(blank_row)
	}
}

fn is_puzzle(sorted: Vec<u16>) -> bool {
	sorted.windows(2).all(|w| w[0] <= w[1])
}

fn is_nxn(matrix: &Vec<Vec<u16>>, msize: usize) -> bool {
	matrix.len() == msize && matrix.iter().all(|row| row.len() == msize)
}

pub fn check_puzzle(matrix: &Vec<Vec<u16>>, msize: usize) -> bool {
	let flatten_matrix: Vec<u16> =
		matrix.iter().flat_map(|row| row.iter()).cloned().collect();
	let (sorted, inversions) =
		inversions::merge_count_inversion(&flatten_matrix);
	is_nxn(matrix, msize)
		&& is_puzzle(sorted)
		&& is_solvable(&flatten_matrix, msize, inversions)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_blank_row() {
		let vec1 = vec![12, 1, 10, 2, 7, 11, 4, 14, 5, 0, 9, 15, 8, 13, 6, 3];
		let vec2 = vec![12, 1, 10, 2, 7, 0, 4, 14, 5, 11, 9, 15, 8, 13, 6, 3];
		let vec3 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
		let vec4 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
		assert_eq!(get_blank_row_from_bottom(&vec1, 4), 2);
		assert_eq!(get_blank_row_from_bottom(&vec2, 4), 3);
		assert_eq!(get_blank_row_from_bottom(&vec3, 3), 3);
		assert_eq!(get_blank_row_from_bottom(&vec4, 4), 4);
	}

	#[test]
	fn test_check_puzzle() {
		let matrix1 = vec![
			vec![1, 2, 3, 4],
			vec![5, 6, 7, 8],
			vec![9, 10, 11, 12],
			vec![13, 14, 15, 0],
		];
		let matrix2 = vec![
			vec![1, 2, 3, 4],
			vec![5, 6, 7, 8],
			vec![9, 10, 11, 12],
			vec![13, 15, 14, 0],
		];
		assert_eq!(check_puzzle(&matrix1, 4), true);
		assert_eq!(check_puzzle(&matrix2, 4), false);
	}
}
