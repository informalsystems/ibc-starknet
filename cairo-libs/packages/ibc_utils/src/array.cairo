pub fn reverse_array<T, +Clone<T>, +Drop<T>>(input: Array<T>) -> Array<T> {
    let mut input_span = input.span();
    let mut reverse = array![];

    while let Some(value) = input_span.pop_back() {
        reverse.append(value.clone());
    }
    reverse
}

pub fn span_contains<T, +PartialEq<T>>(mut span: Span<T>, value: @T) -> bool {
    let mut result = false;
    while let Some(v) = span.pop_front() {
        if v == value {
            result = true;
            break;
        }
    }
    result
}

// ---------------------------------------------------------------
// Implementation of partial ordering for `Array<u8>`
// ---------------------------------------------------------------

pub impl ArrayU8PartialOrd of PartialOrd<Array<u8>> {
    fn le(lhs: Array<u8>, rhs: Array<u8>) -> bool {
        lexicographical_cmp(lhs, rhs) != Ordering::Greater
    }
    fn ge(lhs: Array<u8>, rhs: Array<u8>) -> bool {
        lexicographical_cmp(lhs, rhs) != Ordering::Less
    }
    fn lt(lhs: Array<u8>, rhs: Array<u8>) -> bool {
        lexicographical_cmp(lhs, rhs) == Ordering::Less
    }
    fn gt(lhs: Array<u8>, rhs: Array<u8>) -> bool {
        lexicographical_cmp(lhs, rhs) == Ordering::Greater
    }
}

#[derive(Drop, Debug, PartialEq)]
pub enum Ordering {
    Equal,
    Less,
    Greater,
}

/// Lexicographical comparison of two `u8` arrays.
pub fn lexicographical_cmp(lhs: Array<u8>, rhs: Array<u8>) -> Ordering {
    let lhs_len = lhs.len();
    let rhs_len = rhs.len();
    let mut lhs_span = lhs.span();
    let mut rhs_span = rhs.span();

    let mut ordering = Ordering::Equal;

    while let (Some(l), Some(r)) = (lhs_span.pop_front(), rhs_span.pop_front()) {
        if l < r {
            ordering = Ordering::Less;
            break;
        } else if l > r {
            ordering = Ordering::Greater;
            break;
        }
    }

    if ordering != Ordering::Equal {
        return ordering;
    }

    if lhs_len < rhs_len {
        ordering = Ordering::Less
    }

    if lhs_len > rhs_len {
        ordering = Ordering::Greater
    }

    ordering
}
