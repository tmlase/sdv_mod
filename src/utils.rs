use std::cmp::Ordering::{self, *};

/// 用于比较两个版本号的大小，只支持如 1.2.3, 2.3, 3, 三种格式的字符串
pub fn comp_version(ver1: &str, ver2: &str) -> Ordering {
    let mut parts1 = ver1.splitn(3, '.');
    let mut parts2 = ver2.splitn(3, '.');

    loop {
        let optl = parts1.next().map(|s| s.parse::<u32>().unwrap());
        let optr = parts2.next().map(|s| s.parse::<u32>().unwrap());
        if optl.is_none() && optr.is_none() {
            break;
        }
        match optl.unwrap_or(0).cmp(&optr.unwrap_or(0)) {
            Equal => continue,
            or => return or,
        }
    }
    Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp() {
        assert_eq!(comp_version("1.2.3", "1.2.3"), Equal);
        assert_eq!(comp_version("1.2.3", "1.2"), Greater);
        assert_eq!(comp_version("1.2.0", "1.2"), Equal);
        assert_eq!(comp_version("1", "1.2"), Less);
    }
}
