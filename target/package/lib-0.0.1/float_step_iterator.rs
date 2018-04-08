// Basic implementation of an iterator across floats.
// pub makes things exporatable and usable in other files.
pub struct FloatRange {
    current: f64,
    end: f64,
    incr: f64
}

pub fn range(start: f64, finish: f64, step: f64) -> FloatRange {
    FloatRange { current: start, end: finish, incr: step }
}
impl Iterator for FloatRange {
    type Item = f64;
    fn next(&mut self) -> Option <Self::Item> {
        let res = self.current;
        if res >= self.end {
            None
        } else {
            self.current += self.incr;
            Some(res)
        }
    }
}


// through out this main function iterators were used and these were not collected into a f64 vector.
// try to keep a handle on where you have iterators as they can be used without collecting.
//     fn main() {
//         let xs = range(0.0, 1.0, 0.1);
//         let sin_xs = range(0.0, 1.0, 0.1).map(|f| f.sin() );
//         for (x,y) in xs.zip(sin_xs) {
//             println!("sin({:.1}) = {:.3}", x, y );
//         }
//     }
