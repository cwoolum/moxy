match value {
    (0, _) => {}
    [first] => unsafe { first }
    &rest => {}.len()
}
