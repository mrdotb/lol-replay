use super::models::SpectatorEndpoint;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Region {
    KR,
    EUW1,
    NA1,
}

impl Region {
    pub fn to_endpoint(&self) -> SpectatorEndpoint {
        SpectatorEndpoint {
            base_url: base_url(self),
            platform_id: self.platform_id(),
        }
    }
    fn platform_id(&self) -> String {
        self.to_string().to_uppercase()
    }
}

impl FromStr for Region {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kr" => Ok(Region::KR),
            "euw1" => Ok(Region::EUW1),
            "na1" => Ok(Region::NA1),
            _ => Err(format!("'{}' is not a valid region", s)),
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let region_str = match self {
            Region::KR => "kr",
            Region::EUW1 => "euw1",
            Region::NA1 => "na1",
            // ... add more regions as needed
        };
        write!(f, "{}", region_str)
    }
}

fn base_url(region: &Region) -> String {
    format!("http://spectator-consumer.{}.lol.pvp.net:80", region)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_from_str_valid() {
        assert_eq!(Region::from_str("kr"), Ok(Region::KR));
        assert_eq!(Region::from_str("euw1"), Ok(Region::EUW1));
        assert_eq!(Region::from_str("na1"), Ok(Region::NA1));
    }

    #[test]
    fn test_region_from_str_invalid() {
        assert_eq!(
            Region::from_str("invalid"),
            Err("'invalid' is not a valid region".to_string())
        );
        assert_eq!(
            Region::from_str("xyz"),
            Err("'xyz' is not a valid region".to_string())
        );
    }

    #[test]
    fn test_region_display() {
        assert_eq!(format!("{}", Region::KR), "kr");
        assert_eq!(format!("{}", Region::EUW1), "euw1");
        assert_eq!(format!("{}", Region::NA1), "na1");
    }

    #[test]
    fn test_base_url() {
        assert_eq!(
            base_url(&Region::KR),
            "http://spectator-consumer.kr.lol.pvp.net:80"
        );
        assert_eq!(
            base_url(&Region::EUW1),
            "http://spectator-consumer.euw1.lol.pvp.net:80"
        );
        assert_eq!(
            base_url(&Region::NA1),
            "http://spectator-consumer.na1.lol.pvp.net:80"
        );
    }

    #[test]
    fn test_platform_id() {
        assert_eq!(Region::KR.platform_id(), "KR");
        assert_eq!(Region::EUW1.platform_id(), "EUW1");
        assert_eq!(Region::NA1.platform_id(), "NA1");
    }

    #[test]
    fn test_to_endpoint() {
        let kr_endpoint = Region::KR.to_endpoint();
        assert_eq!(
            kr_endpoint.base_url,
            "http://spectator-consumer.kr.lol.pvp.net:80"
        );
        assert_eq!(kr_endpoint.platform_id, "KR");

        let euw1_endpoint = Region::EUW1.to_endpoint();
        assert_eq!(
            euw1_endpoint.base_url,
            "http://spectator-consumer.euw1.lol.pvp.net:80"
        );
        assert_eq!(euw1_endpoint.platform_id, "EUW1");

        let na1_endpoint = Region::NA1.to_endpoint();
        assert_eq!(
            na1_endpoint.base_url,
            "http://spectator-consumer.na1.lol.pvp.net:80"
        );
        assert_eq!(na1_endpoint.platform_id, "NA1");
    }
}
