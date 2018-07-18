use nalgebra::{Matrix, MatrixArray, Vector2, Vector3};

pub trait Element {
    type Node;

    /*
    /// Checks whether the element is legal, e.g. if it is actually ordered
    /// when this is required.
    fn is_legal() -> bool;
    */

    // fn bounding_box();

    // TODO: Replace with impl AsRef when possible.
    fn nodes(&self) -> dyn AsRef<[Self::Node]>;
}



/*
pub struct BoundingBox2<S> {
    pub min: Vector2<S>,
    pub max: Vector2<S>,
}

pub struct BoundingBox3<S> {
    pub min: Vector3<S>,
    pub max: Vector3<S>,
}

pub trait Element2<S> {
    fn bounding_box(&self) -> BoundingBox2<S>;
}

pub trait Element3<S> {
    fn bounding_box(&self) -> BoundingBox3<S>;
}

/// Triangle in ℝ².
pub struct Triangle2<S> {
    /// each column = vertex
    vertices: Matrix<S, U2, U3, MatrixArray<S, U2, U3>>
}

/// Tetrahedron in ℝ³.
pub struct Tetrahedron3<S> {
    /// each column = vertex
    vertices: Matrix<S, U3, U4, MatrixArray<S, U3, U4>>
}

*/
/*
impl<S> Element2<S> for Triangle2<S> where S: Bounded + Float {
    fn bounding_box(&self) -> BoundingBox2 {
        BoundingBox2 {
            min: Vector2::<f64>::from_fn(|i, _| {
                self.vertices.column(i).iter().min()
            }),
            max: Vector2::<f64>::from_fn(|i, _| {
                self.vertices.column(i).iter().max()
            })
        }
    }
}
*/
