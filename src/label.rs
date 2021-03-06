use crate::{Labeling, MakeLabeling, NoLabeling};

pub trait Label {
    type Labeler: Labeling + Clone;
    fn labeler() -> Self::Labeler;
}

impl Label for () {
    type Labeler = NoLabeling;

    fn labeler() -> Self::Labeler {
        NoLabeling
    }
}

// used to help implement macro
// impl Label for i64 {
//     type Labeler = MakeLabeling<i64>;
//
//     fn labeler() -> Self::Labeler {
//         MakeLabeling::<i64>::default()
//     }
// }

macro_rules! primitive_label {
    ($i:ty) => {
        impl Label for $i {
            type Labeler = MakeLabeling<Self>;

            fn labeler() -> Self::Labeler {
                MakeLabeling::<Self>::default()
            }
        }
    };
}

primitive_label!(bool);
primitive_label!(char);
primitive_label!(f32);
primitive_label!(f64);
primitive_label!(i8);
primitive_label!(i32);
primitive_label!(i64);
primitive_label!(i128);
primitive_label!(isize);
primitive_label!(u8);
primitive_label!(u16);
primitive_label!(u32);
primitive_label!(u64);
primitive_label!(u128);
primitive_label!(usize);
primitive_label!(String);

impl<'a> Label for &'a str {
    type Labeler = MakeLabeling<Self>;

    fn labeler() -> Self::Labeler {
        MakeLabeling::<Self>::default()
    }
}
