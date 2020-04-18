fn is_solvable(_matrix: &Vec<Vec<u8>>) -> bool {
	let inv_count: u8 = 0;
	inv_count % 2 == 0
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
