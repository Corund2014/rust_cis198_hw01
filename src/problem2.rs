/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;



fn dot_product(m1:&Matrix,m2:&Matrix, row:usize,column:usize)->f32
{
    let mut result:f32 = 0.0;
    let mat1columns=m1[0].len();
    for i in 0..mat1columns
    {
            let mult=m1[row][i]*m2[i][column];
            result+=mult;
    }
    result
}



/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let mat1rows=mat1.len();
    let mat1columns=mat1[0].len();
    let mat2rows=mat2.len();
    let mat2columns=mat2[0].len();
    assert_eq!(&mat1columns,&mat2rows);
    let mut result_matrix:Matrix=vec![vec![0.0;mat2columns];mat1rows];
    for i in 0..mat1rows as i32
    {
        for j in 0..mat2columns as i32
        {
            let number_ij:f32=dot_product(mat1,mat2,i as usize, j as usize);
            result_matrix[i as usize][j as usize]=number_ij;
        }
    }
    result_matrix
}
