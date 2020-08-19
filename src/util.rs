#[derive(PartialEq)]
#[derive(Debug)]
pub struct Frequency {
    pub text: String
}

impl Frequency {
    pub fn from_packet_string(data: &str) -> Self {
        return Frequency {
            text: format!("1{}.{}", &data[0..2], &data[2..])
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_packet_frequency() {
        let freq = Frequency::from_packet_string(&"23950".to_string());
        assert_eq!(freq.text, "123.950");
    }
}