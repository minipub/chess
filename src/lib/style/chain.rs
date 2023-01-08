use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};

use crate::lib::card::*;
use crate::lib::style::iface::*;

#[derive(Debug)]
pub struct Chain(pub Box<Vec<Card>>);

impl Suit for Chain {
    type Error = &'static str;

    fn suit(&mut self, cs: &Vec<Card>) -> Option<Self::Error> {
        if cs.len() < 5 {
            return Some("not reach 5 elements.");
        }

        let mut v = vec![Card::default(); cs.len()];
        v.clone_from_slice(&cs);

        // println!("before sort, v: {:?}", v);
        v.sort_by(|x, y| x.partial_cmp(y).unwrap());
        // println!("after sort, v: {:?}", v);

        let mut m = v[0].unwrap_point();
        for x in &v {
            let xp = x.unwrap_point();
            if m != xp {
                return Some("not continous.");
            }
            m = xp + 1;
        }

        self.0 = Box::new(v);
        None
    }
}

impl Layer for Chain {
    type Other = Chain;

    fn same_layer(&self, other: Self::Other) -> bool {
        self.0.len() == other.0.len()
    }
}

impl PartialEq for Chain {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0]
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Chain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.gt(other) {
            Some(Ordering::Greater)
        } else if self.lt(other) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }

    fn gt(&self, other: &Self) -> bool {
        self.0[0] > other.0[0]
    }

    fn ge(&self, other: &Self) -> bool {
        self.0[0] >= other.0[0]
    }

    fn le(&self, other: &Self) -> bool {
        !self.gt(other)
    }

    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }
}
