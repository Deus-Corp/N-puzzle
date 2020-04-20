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

fn is_solvable(matrix: &Vec<Vec<u8>>) -> bool {
	let flatten_matrix: Vec<u8> = matrix.iter().flat_map(|row| row.iter()).cloned().collect();
	println!("{:?}", flatten_matrix);
	let inv = get_inversion_count(&flatten_matrix);
	println!("{}", inv);
	inv % 2 == 0
}

fn is_format_coherent(size: u8, matrix: &Vec<Vec<u8>>) -> bool {
	if matrix.len() != size as usize {
		return false;
	}
	for rows in matrix {
		if rows.len() != size as usize {
			return false;
		}
	}
	true
}

pub fn check_puzzle(size: u8, matrix: &Vec<Vec<u8>>) -> bool {
	is_format_coherent(size, matrix) && is_solvable(matrix)
}
