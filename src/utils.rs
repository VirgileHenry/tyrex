const fn arr_insert<const N: usize, T: Copy>(arr: [T; N], elem: T, index: usize) -> [T; N + 1] {
    assert!(index <= N);

    let mut out = [elem; N + 1];
    let mut i = 0;

    while i < index {
        out[i] = arr[i];
        i += 1;
    }

    out[i] = elem;

    while i < N {
        out[i + 1] = arr[i];
        i += 1;
    }

    out
}
