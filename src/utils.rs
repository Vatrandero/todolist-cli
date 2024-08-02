use std::str::FromStr;

use chrono::format::ParseErrorKind;
use chrono::{self, ParseError};
use chrono::prelude::*;
///! Файл с функциями-утилитами.
///! (если к релиз-кандидату не будет хотя бы двух - перенести в main.rs)

/// Принимает строковый материал и 
/// двумя сопособами пытается извечь
/// дату и время.
/// Если в литерале оказалась только дата - подставляеет  время.
/// 
pub fn get_timedate(s: &str) -> Result< chrono::NaiveDateTime, Box<dyn std::error::Error>> { 

    let  f = r"%Y-%m-%d %H:%M";
    dbg!(s); dbg!(f);
    let mut r_dt  = NaiveDateTime::parse_from_str(s, f);
    match r_dt { 
        Ok(dt) => return Ok(dt),
        Err(e) => { 
            // Если ошибка NotEnough - то начит при 
            // создании не указано время.
            // подставим 00:00 и попробуем снова.
        // если не получилось  - мы больше ничгео сделать не можем.
         if let ParseErrorKind::TooShort = e.kind() { 
           let mut s_=  s.to_string(); s_.push_str("00:00");
            r_dt =  NaiveDateTime::parse_from_str(s_.as_str(), f);
            return match r_dt {

                Ok(dt) => Ok(dt),
                Err(e) => Err(e.into())
            }
        }
        // Если ошибка не в чём-то другом - вернём ошибку.
        else {
            return Err(e.into())
        }
    }
}





}