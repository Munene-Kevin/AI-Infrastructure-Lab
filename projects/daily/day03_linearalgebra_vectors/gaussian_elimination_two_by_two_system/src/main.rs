pub type Matrix = Vec<Vec<f64>>;

// Solves a 2x2 system of linear equations using Gaussian elimination
// Input: matrix (2x2 coefficient matrix) and constants (right-hand side vector)
// Output: Result containing the solution vector [x1, x2] or an error
fn solve_system(matrix: &Matrix, constants: &Vec<f64>) -> Result<Vec<f64>, &'static str> {
    // Validate that the matrix is 2x2
    if matrix.len() != 2 || matrix[0].len() != 2 || matrix[1].len() != 2 {
        return Err("Matrix must be a 2 by 2");
    }

    // Create mutable copies of the matrix and constants for manipulation
    let mut aug_matrix = matrix.clone();
    let mut aug_constants = constants.clone();

    // Extract coefficients from the matrix
    let a = aug_matrix[0][0];
    let c = aug_matrix[1][0];

    // Check if pivot element is zero (would cause division by zero)
    if a.abs() < 1e-10 {
        return Err("Pivot element is zero; cannot perform Gaussian elimination");
    }

    // Calculate the factor to eliminate x1 from the second equation
    let factor = c / a;

    // Perform row elimination: R2 = R2 - factor * R1
    for j in 0..2 {
        aug_matrix[1][j] -= factor * aug_matrix[0][j];
    }
    // Update the constant term for the second equation
    aug_constants[1] -= factor * aug_constants[0]; 

    // Check if the system has a unique solution after elimination
    let pivot_second_row = aug_matrix[1][1];
    if pivot_second_row.abs() < 1e-10 {
        return Err("System has no unique solution");
    }

    // Back substitution to find the solutions
    let x2 = aug_constants[1] / pivot_second_row; // Calculate x1 using the first equation
    let x1 = (aug_constants[0] - aug_matrix[0][1] * x2) / a; // Calculate x2 using the second equation

    Ok(vec![x1, x2]) // Return the solution vector
}


//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//test cases
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
#[cfg(test)]
mod tests { 
    use super::*; // Import the solve_system function from the outer scope

    // Helper function to assert that two vectors are approximately equal within a given epsilon
    fn assert_approx_equal(vec1: &Vec<f64>, vec2: &Vec<f64>, epsilon: f64) {
        assert_eq!(vec1.len(), vec2.len(), "Vectors must be of the same length");
        // Compare each element of the vectors and assert that they are approximately equal
        for (a, b) in vec1.iter().zip(vec2.iter()) {
            assert!((a - b).abs() < epsilon, "Expected {:?} to be approximately equal to {:?} within epsilon {}", vec1, vec2, epsilon);
        }
    }

    #[test]
    // Test case for a system with a unique solution
    fn test_solve_system() {
        let matrix = vec![vec![2.0, 1.0], vec![1.0, 1.0]];
        let constants = vec![5.0, 3.0];
        let solution = solve_system(&matrix, &constants).unwrap();
        assert_approx_equal(&solution, &vec![2.0, 1.0], 1e-10);
    }   

    #[test]
    // Test case for a system with no unique solution (parallel lines)
    fn test_solve_system_no_solution() {
        let matrix = vec![vec![1.0, 1.0], vec![2.0, 2.0]];
        let constants = vec![3.0, 6.0];
        let result = solve_system(&matrix, &constants);
        assert!(result.is_err(), "Expected an error for a system with no unique solution");     
    }

    #[test]
    // Test case for a system with infinite solutions (coincident lines)
    fn test_solve_system_infinite_solutions() {
        let matrix = vec![vec![1.0, 1.0], vec![2.0, 2.0]];
        let constants = vec![3.0, 6.0];
        let result = solve_system(&matrix, &constants);
        assert!(result.is_err(), "Expected an error for a system with infinite solutions"); 
    }

    #[test]
    // Test case for a system with a zero matrix (no solutions)
    fn test_solve_system_zero_matrix() {
        let matrix = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        let constants = vec![1.0, 2.0];
        let result = solve_system(&matrix, &constants);
        assert!(result.is_err(), "Expected an error for a system with a zero matrix"); 
    }
}
