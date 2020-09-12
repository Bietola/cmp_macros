#![feature(cmp_min_max_by)]

macro_rules! chain {
    ($fun:expr; $head:expr $(,)*) => {
        $head
    };

    ($fun:expr; $head:expr, $($body:expr),* $(,)*) => {
        $fun(
            $head,
            chain!($fun; $($body,)*),
        )
    };
}

#[macro_export]
macro_rules! max {
    (by_key: $key_fn:expr; $($body:expr),* $(,)*) => {
        chain!(
            |lhs, rhs| std::cmp::max_by_key(lhs, rhs, $key_fn);
            $($body,)*
        );
    };

    ($($body:expr),* $(,)*) => {
        chain!(std::cmp::max; $($body,)*)
    };
}

#[macro_export]
macro_rules! min {
    (by_key: $key_fn:expr; $($body:expr),* $(,)*) => {
        chain!(
            |lhs, rhs| std::cmp::min_by_key(lhs, rhs, $key_fn);
            $($body,)*
        );
    };

    ($($body:expr),* $(,)*) => {
        chain!(std::cmp::min; $($body,)*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn max_by_keys_vec_indexing() {
        let v = vec![3, 2, 1];

        max!(
            by_key: |&x| v[x - 1 as usize];

            1,
            2,
            3,
        );
    }
}
