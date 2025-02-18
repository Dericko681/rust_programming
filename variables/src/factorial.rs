pub trait Factorial {
    fn fact(self) -> Self;
}

impl Factorial for u8 {
    fn fact(self) -> Self {
        if self == 0 {
            1
        }else {
            self*Self::fact(self -1)
        }
    }
}

impl Factorial for u16 {
    fn fact(self) -> Self {
        if self == 0 {
            1
        }else {
            self*Self::fact(self -1)
        }   }
}

impl Factorial for u32 {
    fn fact(self) -> Self {
        if self == 0 {
            1
        }else{

        self*Self::fact( self -1)
        }
    }
}

impl Factorial for u64 {
    fn fact(self) -> Self {
        if self == 0 {
            1
        }else {
             self*Self::fact( self -1)
   
        }
        }
}