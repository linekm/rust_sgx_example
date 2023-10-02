use super::CMov;

impl CMov for bool {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        let a_u8 = *a as u8;
        let b_u8 = *b as u8;
        u8::cnd_select(&a_u8, &b_u8, choice) != 0
    }

    #[inline]
    fn cnd_assign(&mut self, other: &Self, choice: bool) {
        let mut self_u8 = *self as u8;
        let other_u8 = *other as u8;
        self_u8.cnd_assign(&other_u8, choice);
        *self = self_u8 != 0;
    }

    #[inline]
    fn cnd_swap(a: &mut Self, b: &mut Self, choice: bool) {
        let mut a_u8 = *a as u8;
        let mut b_u8 = *b as u8;
        u8::cnd_swap(&mut a_u8, &mut b_u8, choice);
        *a = a_u8 != 0;
        *b = b_u8 != 0;
    }
}
