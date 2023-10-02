use super::*;

macro_rules! impl_tuple {
    ($($index:tt $name:ident)+) => {
        impl<$($name),+> CMov for ($($name,)+)
        where
            $($name: CMov),+
        {
            #[inline]
            fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
                ($(<_ as CMov>::cnd_select(&a.$index, &b.$index, choice),)+)
            }

            #[inline]
            fn cnd_assign(&mut self, other: &Self, choice: bool) {
                $(self.$index.cnd_assign(&other.$index, choice);)+
            }

            #[inline]
            fn cnd_swap(a: &mut Self, b: &mut Self, choice: bool) {
                $(<_ as CMov>::cnd_swap(&mut a.$index, &mut b.$index, choice);)+
            }
        }
    };
}

impl_tuple!(0 T0);
impl_tuple!(0 T0 1 T1);
impl_tuple!(0 T0 1 T1 2 T2);
impl_tuple!(0 T0 1 T1 2 T2 3 T3);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15);

impl CMov for () {
    #[inline]
    fn cnd_select(_a: &Self, _b: &Self, _choice: bool) -> Self {}

    #[inline]
    fn cnd_assign(&mut self, _other: &Self, _choice: bool) {}

    #[inline]
    fn cnd_swap(_a: &mut Self, _b: &mut Self, _choice: bool) {}
}
