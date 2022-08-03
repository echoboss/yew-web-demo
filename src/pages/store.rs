use serde::{Serialize, Deserialize};
use yewdux::prelude::*;

#[derive(Clone,Serialize,Deserialize)]
pub struct AppStore {
    pub count: u32,
}


impl  Default for AppStore {
    fn default() -> Self {
        Self { count: 9 }
    }
}

pub fn init() -> Dispatch<BasicStore<AppStore>> {
    Dispatch::<BasicStore<AppStore>>::new()
}