pub type Matrix = Vec<Vec<f64>>; // defines a type alias for a matrix, which is a vector of vectors of f64 (floating-point numbers). 
                                 // This allows us to use the type Matrix instead of writing Vec<Vec<f64>> every time we want to refer to a matrix.

// (a: &Matrix, b: &Matrix) uses reference to the matrix intead of copying the data
// Result<Matrix, &'static str> is used to return the resulting matrix or an error message if the matrices' dimensions are incompactible
pub fn matrix_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, &'static str> {

    let a_rows = a.len(); // gets the number of rows in matrix a
    let a_cols = a[0].len(); // gets the number of columns in matrix a (assuming all rows have the same number of columns)
    let b_rows = b.len(); // gets the number of rows in matrix b
    let b_cols = b[0].len(); // gets the number of columns in matrix b (assuming all rows have the same number of columns)

    // golden rule of matrix multiplication =noumber of columns in matrix a must equal number of rows in matrix b
    if a_cols != b_rows { // checks if the number of columns in matrix a is equal to the number of rows in matrix b
        return Err("Incompatible dimensions for matrix multiplication"); // prints the error message if the golden rule of matrix multiplication is broken
    }

    let mut result = vec![vec![0.0; b_cols]; a_rows]; // initializes the result matrix with zeros, where the number of rows is equal to 
                                                      // the number of rows in matrix a and the number of columns is equal to the number of columns in matrix b  

    for i in 0..a_rows { // iterates through each row of matrix a
                         // the number of rows of a determines how tall the result matrix is
        for j in 0..b_cols { // iterates through each column of matrix b
                             // the number of columns of b determines how wide the result matrix is
            for k in 0..a_cols { // iterates through each column of matrix a (or equivalently, each row of matrix b) 
                                 // to calculate the dot product for the element at position (i, j) in the result matrix
                result[i][j] += a[i][k] * b[k][j]; // calculates the dot product for the element at position (i, j) in the 
                                                   // result matrix by multiplying the corresponding elements from row i of matrix a 
                                                   // and column j of matrix b and adding them to the current value of result[i][j]
            }
        }
    }

    Ok(result) // returns the resulting matrix if the multiplication is successful
}


//**********************************************
// test module to verify the correctness of the matrix multiplication function. 
// It defines a test case that multiplies two matrices and checks if the result matches the expected output.
//**********************************************

#[cfg(test)] // this attribute indicates that the following module is only compiled and run when testing the code.
mod tests { // defines a module named tests, which contains the test cases for the matrix multiplication function.
    use super::*; // this line allows the test module to access the items defined in the parent module
                  // (in this case, the matrix_multiply function and the Matrix type alias).

    #[tets] // this attribute indicates that the following function is a test case that should be run when testing the code.
    fn test_matrix_multiply() { // defines a test function named test_matrix_multiply, which will be executed as part of the testing process.
        let a = vec![ // defines matrix a as vector of vectors of f64 (floating-point numbers). 
                     // In this case, matrix a has 2 rows and 3 columns.
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],    
        ];

        let b = vec![ // defines matrix b as vector of vectors of f64 (floating-point numbers). 
                     // In this case, matrix b has 3 rows and 2 columns.
            vec![7.0, 8.0],
            vec![9.0, 10.0],
            vec![11.0, 12.0],
        ];

        let expected = vec![ // defines the expected result of multiplying matrix a by matrix b. 
                             // The resulting matrix has 2 rows and 2 columns, 
                             // where each element is calculated as the dot product of the corresponding row of matrix a and column of matrix b.
            vec![58.0, 64.0],
            vec![139.0, 154.0],
        ];
        let result = matrix_multiply(&a, &b).unwrap(); // calls the matrix_multiply function with references to matrices a and b,
                                                       // and unwraps the result to get the resulting matrix.
        assert_eq!(result, expected); // asserts that the resulting matrix is equal to the expected matrix. 
                                     // If the assertion fails, it will panic and indicate that the test case has failed.
        }

        #[test]
        fn test_incompatible_dimensions() { // defines a test function named test_incompatible_dimensions, 
                                            // which will be executed as part of the testing process.
            let a = vec![ // defines matrix a as vector of vectors of f64 (floating-point numbers). 
                         // In this case, matrix a has 2 rows and 3 columns.
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],    
            ];

            let b = vec![ // defines matrix b as vector of vectors of f64 (floating-point numbers). 
                         // In this case, matrix b has 2 rows and 2 columns, which is incompatible with matrix a for multiplication.
                vec![7.0, 8.0],
                vec![9.0, 10.0],
            ];

            let result = matrix_multiply(&a, &b); // calls the matrix_multiply function with references to matrices a and b,
                                                   // and stores the result in the variable result.

            assert!(result.is_err()); // asserts that the result is an error (i.e., the multiplication failed due to incompatible dimensions).
                                      // If the assertion fails, it will panic and indicate that the test case has failed.
        }

        #[tets]
        fn test_empty_matrices() { // defines a test function named test_empty_matrices, 
                                    // which will be executed as part of the testing process.
            let a: Vec<Vec<f64>> = vec![]; // defines an empty matrix a as a vector of vectors of f64 (floating-point numbers).
            let b: Vec<Vec<f64>> = vec![]; // defines an empty matrix b as a vector of vectors of f64 (floating-point numbers).

            let result = matrix_multiply(&a, &b).unwrap(); // calls the matrix_multiply function with references to the empty matrices a and b,
                                                           // and unwraps the result to get the resulting matrix.

            assert_eq!(result, vec![]); // asserts that the resulting matrix is equal to an empty vector, 
                                        // which is the expected result when multiplying two empty matrices.
                                        // If the assertion fails, it will panic and indicate that the test case has failed.
        }

        #[test]
        fn test_single_element_matrices() { // defines a test function named test_single_element_matrices, 
                                            // which will be executed as part of the testing process.
            let a = vec![vec![2.0]]; // defines a single-element matrix a as a vector of vectors of f64 (floating-point numbers).
            let b = vec![vec![3.0]]; // defines a single-element matrix b as a vector of vectors of f64 (floating-point numbers).

            let expected = vec![vec![6.0]]; // defines the expected result of multiplying matrix a by matrix b.

            let result = matrix_multiply(&a, &b).unwrap(); // calls the matrix_multiply function with references to matrices a and b,
                                                           // and unwraps the result to get the resulting matrix.

            assert_eq!(result, expected); // asserts that the resulting matrix is equal to the expected matrix. 
                                         // If the assertion fails, it will panic and indicate that the test case has failed.
        }

        #[test]
        fn test_non_square_matrices() { // defines a test function named test_non_square_matrices, 
                                        // which will be executed as part of the testing process.
            let a = vec![ // defines matrix a as vector of vectors of f64 (floating-point numbers). 
                         // In this case, matrix a has 3 rows and 2 columns.
                vec![1.0, 2.0],
                vec![3.0, 4.0],         
                vec![5.0, 6.0],
            ];

            let b = vec![ // defines matrix b as vector of vectors of f64 (floating-point numbers). 
                         // In this case, matrix b has 2 rows and 3 columns.
                vec![7.0, 8.0, 9.0],
                vec![10.0, 11.0, 12.0],
            ];

            let expected = vec![ // defines the expected result of multiplying matrix a by matrix b. 
                                 // The resulting matrix has 3 rows and 3 columns, 
                                 // where each element is calculated as the dot product of the corresponding row of matrix a and column of matrix b.
                vec![27.0, 30.0, 33.0],
                vec![61.0, 68.0, 75.0],
                vec![95.0, 106.0, 117.0],
            ];
            
            let result = matrix_multiply(&a, &b).unwrap(); // calls the matrix_multiply function with references to matrices a and b,
                                                           // and unwraps the result to get the resulting matrix.

            assert_eq!(result, expected); // asserts that the resulting matrix is equal to the expected matrix. 
                                         // If the assertion fails, it will panic and indicate that the test case has failed.
        }

        #[test]
        fn test_identity_matrix() { // defines a test function named test_identity_matrix, 
                                    // which will be executed as part of the testing process.
            let a = vec![ // defines matrix a as vector of vectors of f64 (floating-point numbers). 
                         // In this case, matrix a is a 3x3 identity matrix.
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0],
            ];  

            let b = vec![ // defines matrix b as vector of vectors of f64 (floating-point numbers). 
                         // In this case, matrix b is a 3x3 matrix with arbitrary values.
                vec![2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0],
                vec![8.0, 9.0, 10.0],
            ];

            let expected = b.clone(); // defines the expected result of multiplying matrix a by matrix b, 
                                      // which should be equal to matrix b since a is the identity matrix. 

            let result = matrix_multiply(&a, &b).unwrap(); // calls the matrix_multiply function with references to matrices a and b,
                                                           // and unwraps the result to get the resulting matrix.
            assert_eq!(result, expected); // asserts that the resulting matrix is equal to the expected matrix. 
                                         // If the assertion fails, it will panic and indicate that the test case has failed.      
    }
}