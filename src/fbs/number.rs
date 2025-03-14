use bytes::Bytes;

#[derive(Clone, PartialEq, Debug)]
/// Number with a stable representation
///
/// This number is meant for calculations, but for communication.
///
pub struct NumberBuffer {
    pub value: String,
}

/// Format the number with the specified decimal places
///
fn format_number<A: Into<f64>>(value: A, decimal_number: usize) -> String {
    format!("{:.1$}", value.into(), decimal_number as usize)
}

impl NumberBuffer {
    ///
    ///
    pub fn from_raw_data(raw_data: Bytes) -> Self {
        Self {
            value: String::from_utf8(raw_data.to_vec()).unwrap(),
        }
    }

    ///
    ///
    pub fn raw_data(&self) -> Bytes {
        let mut v = serde_json::to_vec(&self.value).unwrap();
        v.remove(0);
        v.pop();
        Bytes::from(v)
    }

    /// Control the number of decimals to keep
    ///
    pub fn from_float_with_decimals<A: Into<f64>>(value: A, decimals: usize) -> Self {
        Self {
            value: format_number(value, decimals),
        }
    }

    /// Convert into a i32
    ///
    pub fn try_into_i32(&self) -> Result<i32, String> {
        self.value
            .parse()
            .map_err(|_| format!("Cannot parse {:?} into i32", &self.value))
    }

    /// Convert into a f32
    ///
    pub fn try_into_f32(&self) -> Result<f32, String> {
        self.value
            .parse()
            .map_err(|_| format!("Cannot parse {:?} into f32", &self.value))
    }

    /// Convert into a f64
    ///
    pub fn try_into_f64(&self) -> Result<f32, String> {
        self.value
            .parse()
            .map_err(|_| format!("Cannot parse {:?} into f64", &self.value))
    }
}

/// Allow implicit convertion
///
impl From<f32> for NumberBuffer {
    fn from(value: f32) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

/// Allow implicit convertion
///
impl From<f64> for NumberBuffer {
    fn from(value: f64) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

/// Allow implicit convertion
///
impl From<u16> for NumberBuffer {
    fn from(value: u16) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

/// Allow implicit convertion
///
impl From<u32> for NumberBuffer {
    fn from(value: u32) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

/// Allow implicit convertion
///
impl From<i32> for NumberBuffer {
    fn from(value: i32) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}
