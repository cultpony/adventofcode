pub struct StaticMatrix<const N: usize, const M: usize, T> {
    data: [[T; N]; M],
}

pub struct DynMatrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> From<Vec<Vec<T>>> for DynMatrix<T> {
    fn from(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for DynMatrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("DynMatrix");
        for (iidx, idat) in self.data.iter().enumerate() {
            let mut idatbuf = Vec::new();
            for jdat in idat {
                idatbuf.push(jdat.to_string());
            }
            let idatbuf = idatbuf.join(" ");
            ds.field(&iidx.to_string(), &idatbuf);
        }
        ds.finish()
    }
}

pub trait Matrix {
    type Data;
    /// Returns true if X and Y are in-bounds
    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.size_x() && y < self.size_y()
    }
    /// SAFETY
    ///
    /// Panics when X or Y are out of bounds
    fn get(&self, x: usize, y: usize) -> Option<&Self::Data>;
    /// SAFETY
    ///
    /// panics when X or Y are out of bounds
    fn set(&mut self, x: usize, y: usize, data: Self::Data) -> Option<Self::Data>;
    fn size_x(&self) -> usize;
    fn size_y(&self) -> usize;
}

impl<const N: usize, const M: usize, T> Matrix for StaticMatrix<N, M, T> {
    type Data = T;

    fn get(&self, x: usize, y: usize) -> Option<&Self::Data> {
        self.data.get(x).and_then(|row| row.get(y))
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < N && y < M
    }

    fn set(&mut self, x: usize, y: usize, data: Self::Data) -> Option<Self::Data> {
        self.data
            .get_mut(x)
            .and_then(|row| row.get_mut(y))
            .map(|old| std::mem::replace(old, data))
    }

    fn size_x(&self) -> usize {
        N
    }

    fn size_y(&self) -> usize {
        M
    }
}

impl<T> Matrix for DynMatrix<T> {
    type Data = T;

    fn get(&self, x: usize, y: usize) -> Option<&Self::Data> {
        self.data.get(x).and_then(|row| row.get(y))
    }

    fn set(&mut self, x: usize, y: usize, data: Self::Data) -> Option<Self::Data> {
        self.data
            .get_mut(x)
            .and_then(|row| row.get_mut(y))
            .map(|old| std::mem::replace(old, data))
    }

    fn size_x(&self) -> usize {
        self.data.len()
    }

    fn size_y(&self) -> usize {
        self.data.first().map(|ylen| ylen.len()).unwrap()
    }
}
