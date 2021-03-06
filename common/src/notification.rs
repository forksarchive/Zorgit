use std::borrow::Borrow;
use std::str::FromStr;
use rocket::http::RawStr;
use rocket::request::{FromFormValue, FromParam};


#[derive(Clone, Debug, PartialEq)]
pub enum Notification {
    Disabled,
    Enabled,
    OnMentions,
}

impl Notification {
    pub fn to_i16(&self) -> i16 {
        self.into()
    }
}

impl<'a> FromFormValue<'a> for Notification {
    type Error = &'a RawStr;

    fn from_form_value(form_value: &'a RawStr) -> Result<Notification, Self::Error> {
        form_value.parse::<Notification>()
            .map_err(|_| form_value)
    }
}

impl<'a> FromParam<'a> for Notification {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Notification, Self::Error> {
        param.parse::<Notification>()
            .map_err(|_| param)
    }
}

impl FromStr for Notification {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Notification, Self::Err> {
        match s.to_lowercase().as_ref() {
            "disabled" => Ok(Notification::Disabled),
            "enabled" => Ok(Notification::Enabled),
            "onmentions" => Ok(Notification::OnMentions),
            _ => {
                let number = s.parse::<i16>()?;
                Ok(Notification::from(number))
            },
        }
    }
}

impl From<i16> for Notification {
    fn from(number: i16) -> Self {
        match number {
            0 => Notification::Disabled,
            1 => Notification::Enabled,
            2 => Notification::OnMentions,
            _ => panic!("Unknown type of Notification"),
        }
    }
}

impl Into<i16> for &Notification {
    fn into(self) -> i16 {
        match self {
            Notification::Disabled => 0,
            Notification::Enabled => 1,
            Notification::OnMentions => 2,
        }
    }
}

impl Into<i16> for Notification {
    fn into(self) -> i16 {
        self.borrow().into()
    }
}

impl<'f> From<&'f str> for Notification {
    fn from(value: &'f str) -> Self {
        match value.to_lowercase().as_ref() {
            "disabled" => Notification::Disabled,
            "enabled" => Notification::Enabled,
            "onmentions" => Notification::OnMentions,
            _ => {
                let number = value.parse::<i16>().expect("Unknown type of Notification");
                Notification::from(number)
            },
        }
    }
}

impl From<String> for Notification {
    fn from(value: String) -> Self {
        Notification::from(&value)
    }
}

impl From<&String> for Notification {
    fn from(value: &String) -> Self {
        let val: &str = value.borrow();
        Notification::from(val)
    }
}