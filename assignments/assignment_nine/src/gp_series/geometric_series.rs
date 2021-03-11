///CustomIterator trait
pub trait CustomIterator {
    fn gp_gen(&mut self) -> Vec<i32>;
}
///GeometricSeries Structure.
///
/// #fields
///
/// first_number : its the first number of series.
/// current_number : its the current number of series that we need to find.
/// ratio : constant factor between consecutive terms.
pub struct GeometricSeries {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}
///Implementing CustomIterator Trait for GeometricSeries Structure
impl CustomIterator for GeometricSeries {
    /// gp_gen function find the geometric series.
    ///
    /// #Arguments
    ///
    /// &mut self.
    ///
    /// #Return
    ///
    /// Returns Vector<i32> having geometric series
    fn gp_gen(&mut self) -> Vec<i32> {
        let mut gp_series: Vec<i32> = Vec::new();
        for i in 0..11 {
            self.current_number = self.first_number * self.ratio.pow(i);
            gp_series.push(self.current_number);
        }
        gp_series
    }
}
