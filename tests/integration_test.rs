use your_literals::c_style;

#[test]
fn it_works() {
    c_style!(constants => {
        #define FRED: i8 => 0x7F,
        #define PLUGH: i32 => 0x7FFF_FFFF,
    });

    assert_eq!(FRED, fred!());
    assert_eq!(PLUGH, plugh!());
}
