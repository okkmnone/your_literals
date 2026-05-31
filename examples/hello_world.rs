mod konsts {
    use your_literals::c_style;

    c_style!(export pub(crate) constants => {
        #define BAZ: &str => "baz",
        #define QUX: u8 => 200,
    });
}

fn main() {
    assert_eq!(konsts::BAZ, baz!());
    assert_eq!(konsts::QUX, qux!());
}
