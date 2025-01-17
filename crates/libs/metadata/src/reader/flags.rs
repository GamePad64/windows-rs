#[derive(Default)]
pub struct ParamFlags(pub u32);

impl ParamFlags {
    pub fn input(&self) -> bool {
        self.0 & 0x0001 != 0
    }
    pub fn output(&self) -> bool {
        self.0 & 0x0002 != 0
    }
    pub fn optional(&self) -> bool {
        self.0 & 0x0010 != 0
    }
}

#[derive(Default)]
pub struct PInvokeAttributes(pub u32);

impl PInvokeAttributes {
    pub fn last_error(&self) -> bool {
        self.0 & 0x0040 != 0
    }
}
