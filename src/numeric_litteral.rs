#[derive(Debug, PartialEq, Clone)]
pub enum NumericLiteral {
    Float(u8, f64),

    SysUint(u64),
    Uint(u8, u64),
    SysInt(i64),
    Int(u8, i64),

    Boolean(bool),
}

fn parse_atomic_floating_point(s: &str) -> anyhow::Result<f64> {
    Ok(s.parse::<f64>()?)
}

fn parse_atomic_integer(s: &str) -> anyhow::Result<i64> {
    Ok(s.parse::<i64>()?)
}

fn parse_atomic_unsigned(s: &str) -> anyhow::Result<u64> {
    Ok(s.parse::<u64>()?)
}

fn parse_f(s: &str) -> anyhow::Result<f64> {
    Ok(if let Some((a, b)) = s.split_once('E') {
        let a = parse_atomic_floating_point(a)?;
        let b = parse_atomic_integer(b)?;
        a * 10f64.powi(b as i32)
    } else {
        parse_atomic_floating_point(s)?
    })
}
fn parse_u(s: &str) -> anyhow::Result<u64> {
    Ok(if let Some((a, b)) = s.split_once('E') {
        let a = parse_atomic_unsigned(a)?;
        let b = parse_atomic_unsigned(b)?;
        a * 10u64.pow(b as u32)
    } else {
        parse_atomic_unsigned(s)?
    })
}
fn parse_i(s: &str) -> anyhow::Result<i64> {
    Ok(if let Some((a, b)) = s.split_once('E') {
        let a = parse_atomic_integer(a)?;
        let b = parse_atomic_unsigned(b)?;
        a * 10i64.pow(b as u32)
    } else {
        parse_atomic_integer(s)?
    })
}

enum ExtractSignatureAndVolumeResult {
    Signature(char),
    SignatureAndVolume(char, u8),
}

fn extract_signature_and_volume_and_base(
    s: &str,
) -> anyhow::Result<(String, ExtractSignatureAndVolumeResult)> {
    use ExtractSignatureAndVolumeResult::*;
    let mut signature = None;
    let mut volume = String::new();
    let mut return_string = String::new();

    for char in s.chars() {
        if let Some(_) = signature {
            volume.push(char);
        } else {
            match char {
                'd' => return Ok((return_string, SignatureAndVolume('f', 64))),
                'b' => return Ok((return_string, Signature('b'))),

                'f' => signature = Some('f'),
                'i' => signature = Some('i'),
                'u' => signature = Some('u'),
                _ => return_string.push(char),
            }
        }
    }

    match signature {
        Some(signature) => Ok((
            return_string,
            if volume.len() == 0 {
                if signature == 'f' {
                    SignatureAndVolume('f', 32)
                } else {
                    Signature(signature)
                }
            } else {
                SignatureAndVolume(signature, volume.parse::<u8>()?)
            },
        )),
        None => Ok((return_string, Signature('i'))),
    }
}

impl std::str::FromStr for NumericLiteral {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (s, vol_sig) = extract_signature_and_volume_and_base(s)?;

        use ExtractSignatureAndVolumeResult::*;
        use NumericLiteral::*;

        match vol_sig {
            Signature('u') => Ok(SysUint(parse_u(s.as_str())?)),
            Signature('i') => Ok(SysInt(parse_i(s.as_str())?)),
            Signature('b') => Ok(Boolean(s != "0")),

            SignatureAndVolume('u', volume) => Ok(Uint(volume, parse_u(s.as_str())?)),
            SignatureAndVolume('i', volume) => Ok(Int(volume, parse_i(s.as_str())?)),
            SignatureAndVolume('f', volume) => Ok(Float(volume, parse_f(s.as_str())?)),

            _ => panic!("Unimplemented literal"),
        }
    }
}

impl ToString for NumericLiteral {
    fn to_string(&self) -> String {
        use NumericLiteral::*;
        match *self {
            Float(size, n) => format!("{}f{}", n, size),
            SysUint(n) => format!("{}u", n),
            SysInt(n) => format!("{}i", n),
            Uint(size, n) => format!("{}u{}", n, size),
            Int(size, n) => format!("{}i{}", n, size),
            Boolean(true) => "1b".to_string(),
            Boolean(false) => "0b".to_string(),
        }
    }
}
