
#[repr(u8)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,serde::Deserialize, serde::Serialize)]
pub enum Cell{
    Dead = 0,
    Alive = 1
}