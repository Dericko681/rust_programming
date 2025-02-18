
#[derive(Debug, Clone)]
pub enum Labelling{
    Numeric,
    Alphanumeric,
    Alphabetic,
}

impl From<&str> for Labelling{
fn from(value:&str)->Self{
    match value.to_lowercase().as_str(){
        "alphanumeric" => Labelling::Alphanumeric,
        "alphabetic" => Labelling::Alphabetic,
        _ => Labelling::Numeric,
    }
}
}


impl Into<&str> for Labelling{
    fn into(self) -> &'static str{
        match self{
            Self::Alphanumeric => "alphanumeric",
            Self::Alphabetic => "alphabetic",
            Self::Numeric => "Numeric",
        }
    }

}