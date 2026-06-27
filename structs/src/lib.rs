mod array;

#[cfg(test)]
mod tests {
    use crate::array::Array;

    fn data() -> Array<i64> {
        let mut res = Array::<i64>::new();
        res.push(4);
        res.push(6);
        res.push(7);
        res.push(3);
        res.push(9);
        res
    }
    #[test]
    fn to_iter() {
        let res = data();
        let mut iter = res.clone().into_iter();
        assert_eq!(Some(4), iter.next());
        assert_eq!(Some(6), iter.next());
        assert_eq!(Some(7), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(9), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn index() {
        let mut res = data();
        assert_eq!(res[1], 6);
        assert_eq!(9, res.pop());
        assert_eq!(Some(3), res.last());
        assert_eq!(Some(4), res.first());
    }

    #[test]
    fn loop_on() {
        let res = data();
        let mut ind = 0 as usize;
        for el in res.clone() {
            assert_eq!(res[ind], el);
            ind += 1;
        }
    }

    #[test]
    fn capacity() {
        let mut res = Array::<i64>::with_capacity(4);
        res.push(3);
        res.push(2);
        res.push(8);
        res.push(9);
        res.push(7);
        assert_eq!(8, res.capacity());
        res.pop();
        assert_eq!(4, res.capacity());
        res.pop();
        assert_eq!(4, res.capacity(), "Capacity should remain 4");
    }

    #[test]
    fn map() {
        let res = data();
        let mut ind = 0;
        for i in res.clone().into_iter().map(|x| x * x) {
            let d = res[ind];
            assert_eq!(i, d * d);
            ind += 1;
        }

        let out: Array<i64> = res.clone().into_iter().filter(|x| *x > 5).collect();
        let len = out.len();
        assert_eq!(len, 3);
    }
}
