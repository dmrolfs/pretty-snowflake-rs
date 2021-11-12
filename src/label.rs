use crate::{Labeling, MakeLabeling, NoLabeling};

pub trait Label {
    fn labeler() -> Box<dyn Labeling>;
}

impl Label for () {
    fn labeler() -> Box<dyn Labeling> {
        Box::new(NoLabeling)
    }
}

//used to help implement macro
// impl Label for i64 {
//     fn labeler() -> Box<dyn Labeling> {
//         Box::new(MakeLabeling::<i64>::default())
//     }
// }

macro_rules! primitive_label {
    ($i:ty) => {
        impl Label for $i {
            fn labeler() -> Box<dyn Labeling> {
                Box::new(MakeLabeling::<$i>::default())
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
primitive_label!(&str);
primitive_label!(u8);
primitive_label!(u16);
primitive_label!(u32);
primitive_label!(u64);
primitive_label!(u128);
primitive_label!(usize);
primitive_label!(String);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_unit_labeler() {
        let actual = <dyn Labeling>::summon::<()>();
        assert_eq!(actual.label().as_ref(), "");
    }

    #[test]
    fn test_i64_labeler() {
        let l = i64::labeler();
        let actual = l.label();
        assert_eq!(actual.as_ref(), "i64");
    }

    #[test]
    fn test_str_labeler() {
        let actual = <dyn Labeling>::summon::<&str>().label();
        assert_eq!(actual.as_ref(), "&str");
    }
}
