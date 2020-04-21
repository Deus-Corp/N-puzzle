fn get_inversion_count(arr: &Vec<u8>) -> usize {
	let mut inversions = 0;
	for i in 0..arr.len() - 1 {
		for j in (i + 1)..arr.len() {
			if arr[j] != 0 && arr[i] != 0 && arr[i] > arr[j] {
				inversions += 1;
			}
		}
	}
	inversions
}

fn get_blank_index(arr: &Vec<u8>) -> usize {
	for i in 0..arr.len() {
		if arr[i] == 0 {
			return i;
		}
	}
	panic!("Why there is no blank ?!");
}

fn get_blank_row_from_bottom(mflat: &Vec<u8>, msize: usize) -> usize {
	msize - get_blank_index(mflat) / msize
}

fn is_even(n: usize) -> bool {
	n % 2 == 0
}

// [https://www.cs.bham.ac.uk/~mdr/teaching/modules04/java2/TilesSolvability.html]
//
// Formula for determining solvability:
// ((grid width is odd) && (# is even)) || ((grid width is even) && ((blank is on odd row from bottom) == (# is even)))
//
fn is_solvable(mflat: &Vec<u8>, msize: usize) -> bool {
	let inversions = get_inversion_count(mflat);
	if is_even(msize) {
		let blank_row = get_blank_row_from_bottom(mflat, msize);
		!is_even(blank_row) == is_even(inversions)
	} else {
		is_even(inversions)
	}
}

fn is_format_coherent(matrix: &Vec<Vec<u8>>, msize: usize) -> bool {
	matrix.len() == msize as usize && matrix.iter().all(|row| row.len() == msize as usize)
}

pub fn check_puzzle(matrix: &Vec<Vec<u8>>, msize: usize) -> bool {
	let flatten_matrix: Vec<u8> = matrix.iter().flat_map(|row| row.iter()).cloned().collect();
	is_format_coherent(matrix, msize) && is_solvable(&flatten_matrix, msize)
}
