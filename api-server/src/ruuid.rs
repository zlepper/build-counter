//use rocket::http::RawStr;
//use rocket::request::{FromFormValue, FromParam};
//use std::fmt;
//use std::str::FromStr;
//use uuid::{ParseError, Uuid};
//
//// A wrapper around Uuid, so it can be used with Rocket
//#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
//pub struct RUuid(Uuid);
//
//impl Into<Uuid> for RUuid {
//    fn into(self) -> Uuid {
//        self.into_inner()
//    }
//}
//
//impl RUuid {
//    pub fn into_inner(self) -> Uuid {
//        self.0
//    }
//}
//impl<'a> FromParam<'a> for RUuid {
//    type Error = ParseError;
//
//    /// A value is successfully parsed if `param` is a properly formatted Uuid.
//    /// Otherwise, a `ParseError` is returned.
//    #[inline(always)]
//    fn from_param(param: &'a RawStr) -> Result<RUuid, Self::Error> {
//        param.parse()
//    }
//}
//
//impl<'v> FromFormValue<'v> for RUuid {
//    type Error = &'v RawStr;
//
//    /// A value is successfully parsed if `form_value` is a properly formatted
//    /// Uuid. Otherwise, the raw form value is returned.
//    #[inline(always)]
//    fn from_form_value(form_value: &'v RawStr) -> Result<RUuid, &'v RawStr> {
//        form_value.parse().map_err(|_| form_value)
//    }
//}
//
//impl FromStr for RUuid {
//    type Err = ParseError;
//
//    #[inline]
//    fn from_str(s: &str) -> Result<RUuid, Self::Err> {
//        Ok(RUuid(s.parse()?))
//    }
//}
//
//impl fmt::Display for RUuid {
//    #[inline(always)]
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        self.0.fmt(f)
//    }
//}
//
//impl PartialEq<Uuid> for RUuid {
//    #[inline(always)]
//    fn eq(&self, other: &Uuid) -> bool {
//        self.0.eq(other)
//    }
//}
//
//#[cfg(test)]
//mod test {
//    use super::FromParam;
//    use super::FromStr;
//    use super::RUuid;
//    use uuid;
//
//    #[test]
//    fn test_from_str() {
//        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
//        let uuid_wrapper = RUuid::from_str(uuid_str).unwrap();
//        assert_eq!(uuid_str, uuid_wrapper.to_string())
//    }
//
//    #[test]
//    fn test_from_param() {
//        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
//        let uuid_wrapper = RUuid::from_param(uuid_str.into()).unwrap();
//        assert_eq!(uuid_str, uuid_wrapper.to_string())
//    }
//
//    #[test]
//    fn test_into_inner() {
//        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
//        let uuid_wrapper = RUuid::from_param(uuid_str.into()).unwrap();
//        let real_uuid: uuid::Uuid = uuid_str.parse().unwrap();
//        let inner_uuid: uuid::Uuid = uuid_wrapper.into_inner();
//        assert_eq!(real_uuid, inner_uuid)
//    }
//
//    #[test]
//    fn test_partial_eq() {
//        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2";
//        let uuid_wrapper = RUuid::from_param(uuid_str.into()).unwrap();
//        let real_uuid: uuid::Uuid = uuid_str.parse().unwrap();
//        assert_eq!(uuid_wrapper, real_uuid)
//    }
//
//    #[test]
//    #[should_panic(expected = "InvalidLength")]
//    fn test_from_param_invalid() {
//        let uuid_str = "c1aa1e3b-9614-4895-9ebd-705255fa5bc2p";
//        RUuid::from_param(uuid_str.into()).unwrap();
//    }
//}
