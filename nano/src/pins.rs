use super::pin::*;
use super::port::*;
pub struct Pins{
    pub d0:Pin::<PortD, 0, Input>,
    pub d1:Pin::<PortD, 1, Input>,
    pub d2:Pin::<PortD, 2, Input>,
    pub d3:Pin::<PortD, 3, Input>,
    pub d4:Pin::<PortD, 4, Input>,
    pub d5:Pin::<PortD, 5, Input>,
    pub d6:Pin::<PortD, 6, Input>,
    pub d7:Pin::<PortD, 7, Input>,

    pub d8:Pin::<PortB, 0, Input>,
    pub d9:Pin::<PortB, 1, Input>,
    pub d10:Pin::<PortB, 2, Input>,
    pub d11:Pin::<PortB, 3, Input>,
    pub d12:Pin::<PortB, 4, Input>,
    pub d13:Pin::<PortB, 5, Input>,

    pub a0:Pin::<PortC, 0, Input>,
    pub a1:Pin::<PortC, 1, Input>,
    pub a2:Pin::<PortC, 2, Input>,
    pub a3:Pin::<PortC, 3, Input>,
    pub a4:Pin::<PortC, 4, Input>,
    pub a5:Pin::<PortC, 5, Input>,
    pub a6:Pin::<PortC, 6, Input>,
    pub a7:Pin::<PortC, 7, Input>,
}
impl Pins{
    pub fn take() -> Self{
        Pins{
            d0:Pin::<PortD, 0, Input>::new(),
            d1:Pin::<PortD, 1, Input>::new(),
            d2:Pin::<PortD, 2, Input>::new(),
            d3:Pin::<PortD, 3, Input>::new(),
            d4:Pin::<PortD, 4, Input>::new(),
            d5:Pin::<PortD, 5, Input>::new(),
            d6:Pin::<PortD, 6, Input>::new(),
            d7:Pin::<PortD, 7, Input>::new(),

            d8:Pin::<PortB, 0, Input>::new(),
            d9:Pin::<PortB, 1, Input>::new(),
            d10:Pin::<PortB, 2, Input>::new(),
            d11:Pin::<PortB, 3, Input>::new(),
            d12:Pin::<PortB, 4, Input>::new(),
            d13:Pin::<PortB, 5, Input>::new(),

            a0:Pin::<PortC, 0, Input>::new(),
            a1:Pin::<PortC, 1, Input>::new(),
            a2:Pin::<PortC, 2, Input>::new(),
            a3:Pin::<PortC, 3, Input>::new(),
            a4:Pin::<PortC, 4, Input>::new(),
            a5:Pin::<PortC, 5, Input>::new(),
            a6:Pin::<PortC, 6, Input>::new(),
            a7:Pin::<PortC, 7, Input>::new(),
        }
    }
}
