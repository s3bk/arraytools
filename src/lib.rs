pub fn array_windows<I, const N: usize>(mut iter: I) -> ArrayWindows<I, N>
where I: Iterator, I::Item: Copy + Default,
{
    let mut buf = [I::Item::default(); N];
    let mut fill = 0;

    for cell in &mut buf[..N-1] {
        if let Some(item) = iter.next() {
            *cell = item;
            fill += 1;
        } else {
            break;
        }
    }

    ArrayWindows {
        iter,
        buf,
        valid: fill == N-1
    }
}
pub struct ArrayWindows<I: Iterator, const N: usize> {
    iter: I,
    buf: [I::Item; N],
    valid: bool,
}
impl<I: Iterator, const N: usize> Iterator for ArrayWindows<I, N> where I::Item: Copy {
    type Item = [I::Item; N];
    fn next(&mut self) -> Option<Self::Item> {
        if !self.valid {
            return None;
        }

        let item = self.iter.next()?;
        let mut buf = self.buf;
        buf[N-1] = item;

        self.buf[..N-1].copy_from_slice(&buf[1..]);
        Some(buf)
    }
}

#[test]
fn test() {
    let mut w = array_windows((0..6));
    assert_eq!(w.next(), Some([0, 1, 2, 3]));
    assert_eq!(w.next(), Some([1, 2, 3, 4]));
    assert_eq!(w.next(), Some([2, 3, 4, 5]));
    assert_eq!(w.next(), None);
}