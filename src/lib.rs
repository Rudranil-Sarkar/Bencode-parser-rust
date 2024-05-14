pub mod bencode {
    use std::collections::BTreeMap;
    use std::fmt;

    pub enum BencodeError {
        ParseStringError(String, String),
        ParseIntgerError(String, String),
        ParseListError(String, String),
        ParseDictError(String, String),
        InvalidElementError(String, String),
    }

    impl fmt::Display for BencodeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::ParseStringError(x, y) => {
                    write!(f, "Failed to Parse Bencode String: {} in {}", x, y)
                }
                Self::ParseIntgerError(x, y) => {
                    write!(f, "Failed to Parse Bencode Integer: {} in {}", x, y)
                }
                Self::ParseListError(x, y) => {
                    write!(f, "Failed to Parse Bencode List: {} in {}", x, y)
                }
                Self::ParseDictError(x, y) => {
                    write!(f, "Failed to Parse Bencode Dict: {} in {}", x, y)
                }
                Self::InvalidElementError(x, y) => {
                    write!(f, "Invalid Element: {} in {}", x, y)
                }
            }
        }
    }

    impl fmt::Debug for BencodeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::ParseStringError(x, y) => {
                    write!(f, "Failed to Parse Bencode String: {} in {}", x, y)
                }
                Self::ParseIntgerError(x, y) => {
                    write!(f, "Failed to Parse Bencode Integer: {} in {}", x, y)
                }
                Self::ParseListError(x, y) => {
                    write!(f, "Failed to Parse Bencode List: {} in {}", x, y)
                }
                Self::ParseDictError(x, y) => {
                    write!(f, "Failed to Parse Bencode Dict: {} in {}", x, y)
                }
                Self::InvalidElementError(x, y) => {
                    write!(f, "Invalid Element: {} in {}", x, y)
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum StringorByteArray {
        StringAble(String),
        NotStringAble(Vec<u8>),
    }
    // Start of Decoding Logic
    // -------------------------------------------------------------------------------------------------

    impl From<StringorByteArray> for BencodeElement {
        fn from(value: StringorByteArray) -> Self {
            BencodeElement::BencodeString(value)
        }
    }

    impl fmt::Display for StringorByteArray {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                StringorByteArray::StringAble(s) => write!(f, "{}", s),
                StringorByteArray::NotStringAble(v) => {
                    let mut print_str = String::from("[");
                    for i in v.iter() {
                        print_str.push_str(format!("{:x}, ", i).as_str())
                    }
                    print_str.pop();
                    print_str.pop();
                    print_str.push(']');
                    write!(f, "{}", print_str)
                }
            }
        }
    }

    #[derive(Debug, Clone)]

    pub enum BencodeElement {
        BencodeInteger(i64),
        BencodeString(StringorByteArray),
        BencodeList(Vec<BencodeElement>),
        BencodeDict(BTreeMap<String, BencodeElement>),
    }

    impl TryInto<BTreeMap<String, BencodeElement>> for BencodeElement {
        type Error = ();
        fn try_into(self) -> std::result::Result<BTreeMap<String, BencodeElement>, Self::Error> {
            if let BencodeElement::BencodeDict(map) = self {
                Ok(map)
            } else {
                Err(())
            }
        }
    }

    impl TryInto<i64> for BencodeElement {
        type Error = ();
        fn try_into(self) -> std::result::Result<i64, Self::Error> {
            if let BencodeElement::BencodeInteger(x) = self {
                Ok(x)
            } else {
                Err(())
            }
        }
    }

    impl TryInto<String> for BencodeElement {
        type Error = &'static str;
        fn try_into(self) -> std::result::Result<String, Self::Error> {
            if let BencodeElement::BencodeString(StringorByteArray::StringAble(x)) = self {
                Ok(x)
            } else {
                Err("Cannot parse as byte array")
            }
        }
    }
    impl TryInto<Vec<BencodeElement>> for BencodeElement {
        type Error = &'static str;
        fn try_into(self) -> std::result::Result<Vec<BencodeElement>, Self::Error> {
            if let BencodeElement::BencodeList(x) = self {
                Ok(x)
            } else {
                Err("Cannot parse as list")
            }
        }
    }

    impl TryInto<Vec<u8>> for BencodeElement {
        type Error = &'static str;
        fn try_into(self) -> std::result::Result<Vec<u8>, Self::Error> {
            if let BencodeElement::BencodeString(StringorByteArray::NotStringAble(x)) = self {
                Ok(x)
            } else {
                Err("Cannot parse as byte array")
            }
        }
    }

    impl From<Vec<BencodeElement>> for BencodeElement {
        fn from(value: Vec<BencodeElement>) -> Self {
            BencodeElement::BencodeList(value)
        }
    }

    impl fmt::Display for BencodeElement {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::BencodeInteger(x) => {
                    write!(f, "{}", x)
                }
                Self::BencodeString(x) => {
                    write!(f, "\"{}\"", x)
                }

                Self::BencodeDict(x) => {
                    let mut print_str = String::from('{');
                    for (key, value) in x.iter() {
                        print_str.push_str(format!(" {} : {},", key, value).as_str());
                    }
                    print_str.pop();
                    print_str.push_str(" }");
                    write!(f, "{}", print_str)
                }
                Self::BencodeList(x) => {
                    let mut print_str = String::from("[");
                    for i in x.iter() {
                        print_str.push_str(format!("{}, ", i).as_str())
                    }
                    print_str.pop();
                    print_str.pop();
                    print_str.push(']');
                    write!(f, "{}", print_str)
                }
            }
        }
    }

    type Result<T> = core::result::Result<T, BencodeError>;

    fn vectorslice_to_string(value: &[u8]) -> String {
        String::from_utf8_lossy(value).to_string()
    }

    fn decode_bencode_integer(bencode_str: &[u8]) -> Result<(&[u8], i64)> {
        let e_index =
            bencode_str
                .iter()
                .position(|&c| c == b'e')
                .ok_or(BencodeError::ParseIntgerError(
                    "Missing 'e' terminator".to_string(),
                    vectorslice_to_string(bencode_str),
                ))?;
        let integer_str = &bencode_str[1..e_index];
        let parse_int = String::from_utf8(integer_str.to_vec())
            .unwrap()
            .parse::<i64>();

        match parse_int {
            Err(_e) => Err(BencodeError::ParseIntgerError(
                format!(
                    "Could not parse Integer: {}",
                    vectorslice_to_string(integer_str)
                ),
                vectorslice_to_string(bencode_str),
            )),

            Ok(e) => Ok((&bencode_str[..e_index + 1], e)),
        }
    }

    fn decode_bencode_string(bencode_str: &[u8]) -> Result<(&[u8], StringorByteArray)> {
        let colon_index =
            bencode_str
                .iter()
                .position(|&c| c == b':')
                .ok_or(BencodeError::ParseStringError(
                    format!("Missing : in \"{}\"", vectorslice_to_string(bencode_str)).to_string(),
                    "".to_owned(),
                ))?;

        let size_of_str = String::from_utf8(bencode_str[..colon_index].to_vec())
            .unwrap()
            .parse::<usize>();

        if let Ok(x) = size_of_str {
            let size_of_str = x;
            let res_str = &bencode_str[colon_index + 1..=size_of_str + colon_index];

            match std::str::from_utf8(res_str) {
                Ok(x) => Ok((
                    &bencode_str[..=colon_index + res_str.len()],
                    StringorByteArray::StringAble(x.to_string()),
                )),
                Err(_) => Ok((
                    &bencode_str[..=colon_index + res_str.len()],
                    StringorByteArray::NotStringAble(res_str.to_vec()),
                )),
            }
        } else {
            Err(BencodeError::ParseStringError(
                "Invalid String Length".to_owned(),
                "".to_owned(),
            ))
        }
    }

    fn decoder_internal(bencode_str: &[u8]) -> Result<(&[u8], BencodeElement)> {
        match bencode_str.iter().next().unwrap() {
            b'i' => {
                let (parsed, integer) = decode_bencode_integer(bencode_str)?;
                Ok((parsed, BencodeElement::BencodeInteger(integer)))
            }
            b'l' => {
                let mut res: Vec<BencodeElement> = Vec::new();
                let mut rest = bencode_str.split_at(1).1;
                let mut total_parsed_len = 1;

                while !rest.is_empty() && rest[0] != b'e' {
                    let (parsed, bencoded_value) = decoder_internal(rest)?;
                    rest = &rest[parsed.len()..];
                    total_parsed_len += parsed.len();
                    res.push(bencoded_value);
                }

                Ok((
                    &bencode_str[..=total_parsed_len],
                    BencodeElement::BencodeList(res),
                ))
            }
            b'd' => {
                let mut res: BTreeMap<String, BencodeElement> = BTreeMap::new();
                let mut rest = bencode_str.split_at(1).1;
                let mut total_parsed_len = 1;

                let mut key: Option<String> = None;

                while !rest.is_empty() && rest[0] != b'e' {
                    let (parsed, bencoded_value) = decoder_internal(rest)?;
                    rest = &rest[parsed.len()..];
                    total_parsed_len += parsed.len();

                    match key.clone() {
                        Some(x) => {
                            res.insert(x, bencoded_value);
                            key = None
                        }
                        None => match bencoded_value {
                            BencodeElement::BencodeString(StringorByteArray::StringAble(s)) => {
                                key = Some(s);
                            }

                            _ => {
                                return Err(BencodeError::ParseDictError(
                                    "Expected String as key".to_owned(),
                                    vectorslice_to_string(bencode_str),
                                ));
                            }
                        },
                    }
                }

                Ok((
                    &bencode_str[..=total_parsed_len],
                    BencodeElement::BencodeDict(res),
                ))
            }
            b'0'..=b'9' => {
                let (parsed, string) = decode_bencode_string(bencode_str)?;
                Ok((parsed, BencodeElement::BencodeString(string)))
            }
            _ => Err(BencodeError::InvalidElementError(
                "Invalid Bencode String".to_owned(),
                String::from_utf8_lossy(bencode_str).to_string(),
            )),
        }
    }

    pub fn decode_bencode_element(bencode_str: Vec<u8>) -> Result<BencodeElement> {
        Ok(decoder_internal(&bencode_str[..])?.1)
    }

    // -------------------------------------------------------------------------------------------------
    // End of Decoding Logic

    // Start of Encoding Logic
    // -------------------------------------------------------------------------------------------------

    pub enum BencodeEncodeble {
        Number(i64),
        String(StringorByteArray),
        List(Vec<BencodeEncodeble>),
        Dict(BTreeMap<String, BencodeEncodeble>),
    }

    impl Into<BencodeEncodeble> for Vec<u8> {
        fn into(self) -> BencodeEncodeble {
            BencodeEncodeble::String(StringorByteArray::NotStringAble(self))
        }
    }

    impl Into<BencodeEncodeble> for i64 {
        fn into(self) -> BencodeEncodeble {
            BencodeEncodeble::Number(self)
        }
    }

    impl Into<StringorByteArray> for String {
        fn into(self) -> StringorByteArray {
            StringorByteArray::StringAble(self)
        }
    }

    impl Into<StringorByteArray> for Vec<u8> {
        fn into(self) -> StringorByteArray {
            StringorByteArray::NotStringAble(self)
        }
    }

    impl Into<BencodeEncodeble> for &str {
        fn into(self) -> BencodeEncodeble {
            BencodeEncodeble::String(self.to_string().into())
        }
    }

    impl Into<BencodeEncodeble> for String {
        fn into(self) -> BencodeEncodeble {
            BencodeEncodeble::String(self.into())
        }
    }

    impl Into<BencodeEncodeble> for Vec<BencodeEncodeble> {
        fn into(self) -> BencodeEncodeble {
            BencodeEncodeble::List(self)
        }
    }

    impl From<BTreeMap<String, BencodeElement>> for BencodeEncodeble {
        fn from(value: BTreeMap<String, BencodeElement>) -> Self {
            let mut map: BTreeMap<String, BencodeEncodeble> = BTreeMap::new();
            for i in value {
                map.insert(i.0, i.1.into());
            }
            BencodeEncodeble::Dict(map)
        }
    }

    impl From<Vec<BencodeElement>> for BencodeEncodeble {
        fn from(value: Vec<BencodeElement>) -> Self {
            let mut lst: Vec<BencodeEncodeble> = Vec::new();
            for i in value {
                lst.push(i.into());
            }
            BencodeEncodeble::List(lst)
        }
    }

    impl Into<BencodeEncodeble> for BencodeElement {
        fn into(self) -> BencodeEncodeble {
            match self {
                BencodeElement::BencodeDict(dict) => dict.into(),
                BencodeElement::BencodeInteger(int) => BencodeEncodeble::Number(int),
                BencodeElement::BencodeList(lst) => lst.into(),
                BencodeElement::BencodeString(str) => BencodeEncodeble::String(str),
            }
        }
    }

    impl Into<BencodeEncodeble> for BTreeMap<String, BencodeEncodeble> {
        fn into(self) -> BencodeEncodeble {
            BencodeEncodeble::Dict(self)
        }
    }

    fn encode_integer_bencode(i: i64) -> String {
        format!("i{}e", i)
    }

    fn encode_string_bencode(s: &Vec<u8>) -> Vec<u8> {
        let mut len = s.len().to_string();
        let mut res_vec: Vec<u8> = Vec::new();
        len.push(':');

        res_vec.extend(len.bytes());
        res_vec.extend(s);
        res_vec
    }

    pub fn encode_bencode_value(value: &BencodeEncodeble) -> Result<Vec<u8>> {
        match value {
            BencodeEncodeble::Number(num) => Ok(encode_integer_bencode(*num).as_bytes().to_vec()),
            BencodeEncodeble::String(str) => {
                let mut arg: Vec<u8>;
                match str {
                    StringorByteArray::NotStringAble(v) => arg = v.clone(),
                    StringorByteArray::StringAble(s) => arg = s.as_bytes().to_vec(),
                }
                Ok(encode_string_bencode(&arg))
            }
            BencodeEncodeble::List(lst) => {
                let mut res_str: Vec<u8> = Vec::from("l".as_bytes());
                for i in lst {
                    res_str.extend(&encode_bencode_value(i)?)
                }

                res_str.push(b'e');

                Ok(res_str)
            }
            BencodeEncodeble::Dict(dict) => {
                let mut res_str = Vec::from("d".as_bytes());
                for (key, value) in dict.iter() {
                    res_str.extend(encode_bencode_value(&key.clone().into())?);
                    res_str.extend(encode_bencode_value(value)?)
                }
                res_str.push(b'e');
                Ok(res_str)
            }
        }
    }

    // -------------------------------------------------------------------------------------------------
    // End of Encoding Logic
}

#[cfg(test)]
mod test {
    use std::{collections::BTreeMap, fs};

    use crate::bencode::{encode_bencode_value, BencodeEncodeble};

    use self::bencode::{decode_bencode_element, BencodeElement};

    use super::*;

    #[test]
    fn test2() {
        let file_bytes = std::fs::read("sample.torrent").unwrap();

        let decoded = decode_bencode_element(file_bytes).unwrap();

        let test: Vec<BencodeEncodeble> = vec![1.into(), 2.into(), 3.into()].into();

        let x: BTreeMap<String, BencodeElement> = decoded.try_into().unwrap();
        let encoded = bencode::encode_bencode_value(&x.into()).unwrap();
        fs::write("sample.torrent.gen", encoded).unwrap();
        assert!(true)
    }

    // #[test]
    fn test() {
        let lst: Vec<BencodeEncodeble> = vec![
            1.into(),
            2.into(),
            3.into(),
            vec![4.into(), 5.into()].into(),
            "Hello".into(),
            vec![5.into(), 6.into(), vec![7.into(), 8.into()].into()].into(),
        ];

        let mut dict: BTreeMap<String, BencodeEncodeble> = BTreeMap::new();

        let mut dict2: BTreeMap<String, BencodeEncodeble> = BTreeMap::new();
        dict2.insert("hello".to_string(), 123.into());
        dict.insert("test2".to_string(), dict2.into());
        dict.insert("test".to_string(), lst.into());

        let str = bencode::encode_bencode_value(&dict.into()).unwrap();
        assert_eq!(
            str,
            "d4:testli1ei2ei3eli4ei5ee5:Helloli5ei6eli7ei8eeee5:test2d5:helloi123eee"
                .bytes()
                .collect::<Vec<u8>>()
        )

        // let str = "hellohello";
        // assert_eq!(
        //     encode_bencode_value(&str.into()).unwrap(),
        //     "10:hellohello".as_bytes().to_vec()
        // )
    }
}
