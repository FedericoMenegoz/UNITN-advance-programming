pub fn transpose(matrix: ((i32, i32), (i32, i32))) -> ((i32, i32), (i32, i32)) {
    let ((a, b), (c, d)) = matrix;
    ((a, c), (b, d))
}
pub fn es10() {
    let matrix = ((1, 2), (3, 4));
    let transposed = transpose(matrix);
    println!(
        "
Original:
{:?}
{:?}
Transposed:
{:?}
{:?}
",
        matrix.0, matrix.1, transposed.0, transposed.1
    )
}
