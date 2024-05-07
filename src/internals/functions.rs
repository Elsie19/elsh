use crate::internals::variables::ElshLvl;
use crate::internals::variables::Type;

#[derive(Debug)]
// This is for internal functions
pub struct Function {
    name: String,
    // We can func(), in that case the vec would be 0 length
    signature: Vec<Type>,
    lvl: ElshLvl,
}
