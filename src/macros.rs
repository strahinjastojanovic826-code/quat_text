#[macro_export]
macro_rules! quat_text {
    ($($val:expr),* $(,)?) => {
        {
            let mut qt = $crate::QuatText::new();
            $(
                qt.push($val);
            )*
            qt
        }
    };
    ($s:expr) => {
        {
            use std::str::FromStr;
            $crate::QuatText::from_str($s).expect("failed to parse quat text literal")
        }
    };
}