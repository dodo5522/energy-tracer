use super::error::UnitError;
use std::fmt;

/// 物理量の単位を表す値オブジェクト
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Unit(String);

impl Unit {
    /// 物理量単位の値オブジェクトを生成する
    ///
    /// # Arguments
    /// - `name` - 単位を表す文字列
    ///
    /// # Errors
    /// - `UnitError::Empty` - 空文字列が渡された場合
    /// - `UnitError::Blank` - 空白文字列が渡された場合
    /// - `UnitError::Invalid` - 文字列が英数字を含まない、または数字のみの場合
    ///
    pub fn new(name: impl AsRef<str>) -> Result<Self, UnitError> {
        let name = name.as_ref();
        if name.is_empty() {
            Err(UnitError::Empty)
        } else if name.trim().is_empty() {
            Err(UnitError::Blank)
        } else if name.trim().chars().all(|c| c.is_ascii_digit()) {
            Err(UnitError::Invalid(name.into()))
        } else {
            Ok(Self(name.into()))
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<Unit> for String {
    fn from(u: Unit) -> Self {
        u.0
    }
}

impl From<&Unit> for String {
    fn from(u: &Unit) -> Self {
        u.0.to_owned()
    }
}

impl TryFrom<&str> for Unit {
    type Error = UnitError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Unit {
    type Error = UnitError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&String> for Unit {
    type Error = UnitError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests_unit {
    use super::*;

    #[test]
    fn display() {
        let unit = Unit::new("m/s").unwrap();
        assert_eq!(format!("{}", &unit), "m/s");
        assert_eq!(format!("{unit}"), "m/s");
    }

    #[test]
    fn create() {
        assert_eq!(Unit::new("V").unwrap().to_string(), "V");
        assert_eq!(Unit::new("Ah").unwrap().to_string(), "Ah");
        assert_eq!(Unit::new("kWh").unwrap().to_string(), "kWh");
        assert_eq!(Unit::new("m/s").unwrap().to_string(), "m/s");

        assert_eq!(
            format!("{}", Unit::new("").err().unwrap()),
            "unit must not be empty"
        );
        assert_eq!(
            format!("{}", Unit::new(" ").err().unwrap()),
            "unit must not be blank"
        );
        assert_eq!(
            format!("{}", Unit::new("-").err().unwrap()),
            "'-' is invalid"
        );
        assert_eq!(
            format!("{}", Unit::new("123").err().unwrap()),
            "'123' is invalid"
        );
    }

    #[test]
    fn convert() {
        let expected = "kWh";
        let u = Unit::new(expected).unwrap();
        assert_eq!(String::from(&u), expected);
        assert_eq!(String::from(u), expected);

        let u = Unit::new(expected).unwrap();
        let s: String = (&u).into();
        assert_eq!(s, expected);
        let s: String = u.into();
        assert_eq!(s, expected);
    }

    #[test]
    fn try_to_convert() {
        let expected = "m/s";
        let u = Unit::try_from(expected).unwrap();
        assert_eq!(u.0, expected);

        let u: String = Unit::try_from(expected.to_string()).unwrap().into();
        assert_eq!(u, expected);

        let u: String = Unit::try_from(&(expected.to_string())).unwrap().into();
        assert_eq!(u, expected);

        let u: Result<Unit, UnitError> = expected.try_into();
        assert_eq!(u.unwrap().0, expected);

        let u: Result<Unit, UnitError> = expected.to_string().try_into();
        assert_eq!(u.unwrap().0, expected);

        let u: Result<Unit, UnitError> = (&(expected.to_string())).try_into();
        assert_eq!(u.unwrap().0, expected);
    }

    #[test]
    fn try_to_convert_with_error() {
        assert_eq!(Unit::try_from("").err().unwrap(), UnitError::Empty);
        assert_eq!(
            Unit::try_from("".to_string()).err().unwrap(),
            UnitError::Empty
        );
        assert_eq!(
            Unit::try_from(&"".to_string()).err().unwrap(),
            UnitError::Empty
        );
    }
}
