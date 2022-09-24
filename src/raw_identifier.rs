
pub(crate) fn raw_id () {
    let mut r#fn = 1u8;
    println!("{}", r#fn);
    r#fn += 255;
    const MAX_POINTS: u32 = 100_000;
    println!("{} {}", r#fn , MAX_POINTS);
}
