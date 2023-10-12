pub struct Matrix3 {
    matrix: [[f64; 3]; 3],
}

impl Matrix3 {
    pub fn determinant(self) -> f64 {
        (self.matrix[0][0] * self.matrix[1][1] * self.matrix[2][2]
            + self.matrix[0][1] * self.matrix[1][2] * self.matrix[2][0]
            + self.matrix[0][2] * self.matrix[1][0] * self.matrix[2][1])
            - (self.matrix[0][2] * self.matrix[1][1] * self.matrix[2][0]
                + self.matrix[0][1] * self.matrix[1][0] * self.matrix[2][2]
                + self.matrix[0][0] * self.matrix[1][2] * self.matrix[2][1])
    }
}

pub fn get_matrix3(matrix: [[f64; 3]; 3]) -> Matrix3 {
    Matrix3 { matrix }
}

pub struct Matrix2 {
    matrix: [[f64; 2]; 2],
}

impl Matrix2 {
    pub fn determinant(self) -> f64 {
        (self.matrix[0][0] * self.matrix[1][1]) - (self.matrix[0][1] * self.matrix[1][0])
    }
}

pub fn get_matrix2(matrix: [[f64; 2]; 2]) -> Matrix2 {
    Matrix2 { matrix }
}
