use serde::{Deserialize, Serialize};
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptStruct<A: Clone, B: Clone> {
    pub a: Option<A>,
    pub b: Option<B>,
}
pub fn new_optstruct_a<A, B>(input: A) -> OptStruct<A, B>
where
    A: std::clone::Clone,
    B: std::clone::Clone,
{
    return OptStruct {
        a: Some(input),
        b: None,
    };
}
pub fn new_optstruct_b<A, B>(input: B) -> OptStruct<A, B>
where
    A: std::clone::Clone,
    B: std::clone::Clone,
{
    return OptStruct {
        a: None,
        b: Some(input),
    };
}
impl<A: std::clone::Clone, B: std::clone::Clone> OptStruct<A, B> {
    pub fn a(&self) -> Option<&A> {
        return self.a.as_ref();
    }
    #[allow(dead_code)]
    pub fn a_mut(&mut self) -> Option<&mut A> {
        return self.a.as_mut();
    }
    pub fn b(&self) -> Option<&B> {
        return self.b.as_ref();
    }
    pub fn b_mut(&mut self) -> Option<&mut B> {
        return self.b.as_mut();
    }
}
