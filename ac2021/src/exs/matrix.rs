use std::str::FromStr;
use std::cmp::min;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
    n_rows: usize,
    n_columns: usize
}

impl<T> Matrix<T>
    where T: FromStr + Clone, <T as FromStr>::Err: std::fmt::Debug {
    pub fn from_lines(rows: &Vec<String>) -> Matrix<T> {
        let n_rows = rows.len();
        let n_columns = rows.first().unwrap().len();
        let data: Vec<Vec<T>> = rows.iter()
            .map(
                |row|
                    row.split("").filter(|x|x != &"")
                        .map(|height| height.parse::<T>().unwrap())
                        .collect::<Vec<T>>()
            )
            .collect();

        return Matrix { data, n_rows, n_columns }
    }

    pub fn get_point(&self, x: i32, y: i32) -> Option<T> {
        if !Matrix::index_exists(self, x, y) { return None };
        let (x_, y_) = (x as usize, y as usize);
        return self.data.get(y_).unwrap().get(x_).cloned()
    }

    pub fn get_mut_point(&mut self, x: i32, y: i32) -> Option<&mut T> {
        if !Matrix::index_exists(self, x, y) { return None };
        let (x_, y_) = (x as usize, y as usize);
        return self.data.get_mut(y_).unwrap().get_mut(x_)
    }

    pub fn index_exists(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 { return false };
        let (x_, y_) = (x as usize, y as usize);
        if x_ >= self.n_columns || y_ >= self.n_rows { return false };
        return true
    }

    pub fn n_rows(&self) -> usize {
        return self.n_rows
    }

    pub fn n_columns(&self) -> usize {
        return self.n_columns
    }

    pub fn points_iter(&self) -> impl Iterator<Item=T> + '_ {
        let row_ids = 0..(self.n_rows);
        let col_ids = 0..(self.n_columns);
        return row_ids.flat_map(move |y| col_ids.clone().map(move |x| (x, y)))
            .map(move |(x, y)| self.data.get(y).unwrap().get(x).unwrap())
            .map(|p| p.clone())
    }

    // TODO: this needs a better name
    pub fn sub_matrix(
        &self,
        start_at_x: usize,
        start_at_y: usize,
        length: usize
    ) -> Option<Matrix<T>> {
        if (start_at_x >= self.n_columns) || (start_at_y >= self.n_rows) {
            return None
        }

        let end_at_x = min(start_at_x + length, self.n_columns);
        let end_at_y = min(start_at_y + length, self.n_rows);
        let n_rows = end_at_y - start_at_y;
        let n_columns = end_at_x - start_at_x;
        let mut data = Vec::with_capacity(n_rows + 1);
        let source_rows = &self.data[start_at_y..end_at_y];
        for row in source_rows {
            data.push(row[start_at_x..end_at_x].to_owned())
        }
        return Some(Matrix { data, n_rows, n_columns })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exs::utils::strs_to_strings;

    #[test]
    fn test_matrix() {
        let input = strs_to_strings(&vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581"
        ]);
        let mut m = Matrix::<u32>::from_lines(&input);
        assert_eq!(m.n_rows(), 10);
        assert_eq!(m.n_columns(), 10);
        assert_eq!(m.get_point(1, 1), Some(3));
        assert_eq!(m.get_point(100, 100), None);
        assert_eq!(m.get_point(-100, -100), None);
        assert_eq!(m.points_iter().sum::<u32>(), 382);

        let p = m.get_mut_point(1, 1).unwrap();
        *p = 42;
        assert_eq!(m.get_point(1, 1), Some(42));
        assert_eq!(m.points_iter().sum::<u32>(), 421);
        assert!(m.index_exists(1, 1));
        assert!(!m.index_exists(100, 100));
    }

    #[test]
    fn test_sub_matrix() {
        let input = strs_to_strings(&vec![
            "11637",
            "13819",
            "21365"
        ]);
        let m = Matrix::<u32>::from_lines(&input);
        assert_eq!(m.sub_matrix(100, 100, 1), None);
        assert_eq!(m.sub_matrix(0, 0, 100), Some(m.clone()));
        assert_eq!(
            m.sub_matrix(1, 1, 2),
            Some(
                Matrix::<u32>::from_lines(&strs_to_strings(&vec![
                    "38",
                    "13"
                ]))
            )
        );
        assert_eq!(
            m.sub_matrix(1, 1, 200),
            Some(
                Matrix::<u32>::from_lines(&strs_to_strings(&vec![
                    "3819",
                    "1365"
                ]))
            )
        );
        assert_eq!(
            m.sub_matrix(4, 2, 200),
            Some(
                Matrix::<u32>::from_lines(&strs_to_strings(&vec![
                    "5"
                ]))
            )
        );
    }
}
