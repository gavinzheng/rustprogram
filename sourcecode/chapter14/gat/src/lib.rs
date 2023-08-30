struct WindowsMut<'t, T> {
    slice: &'t mut [T],
    start: usize,
    window_size: usize,
}
trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
impl<'t, T> Iterator for WindowsMut<'t, T> {
    type Item = &'t mut [T];

    fn next<'a>(&'a mut self) -> Option<Self::Item> {
        let retval = self.slice[self.start..].get_mut(..self.window_size)?;
        self.start += 1;
        Some(retval)
    }
}

// impl<'t, T> LendingIterator for WindowsMut<'t, T> {
//     type Item<'a> where Self: 'a = &'a mut [T];

//     fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
//         let retval = self.slice[self.start..].get_mut(..self.window_size)?;
//         self.start += 1;
//         Some(retval)
//     }
// }