use super::*;
use core::{fmt, mem};

/// Option<T> backed by CMOV.
#[derive(Clone, CMov)]
pub struct CndOption<T: CMov> {
    value: T,
    is_some: bool,
}

impl<T: CMov + Copy> Copy for CndOption<T> {}

impl<T: CMov + fmt::Debug> fmt::Debug for CndOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CndOption")
            .field("value", &self.value)
            .field("is_some", &self.is_some)
            .finish()
    }
}

impl<T: CMov + Default> Default for CndOption<T> {
    #[inline]
    fn default() -> Self {
        Self::new_none()
    }
}

impl<T: CMov> From<CndOption<T>> for Option<T> {
    #[inline]
    fn from(input: CndOption<T>) -> Self {
        if input.is_some {
            Option::Some(input.value)
        } else {
            None
        }
    }
}

impl<T: CMov + Default> From<Option<T>> for CndOption<T> {
    #[inline]
    fn from(input: Option<T>) -> Self {
        match input {
            Some(value) => Self {
                value,
                is_some: true,
            },
            None => Self::default(),
        }
    }
}

impl<T: CMov + PartialEq> PartialEq for CndOption<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_some != other.is_some {
            return false;
        }

        if self.is_some {
            self.value == other.value
        } else {
            true
        }
    }
}

impl<T: CMov> CndOption<T> {
    #[inline]
    pub fn new(value: T, is_some: bool) -> Self {
        Self { value, is_some }
    }

    #[inline]
    pub fn new_some(value: T) -> Self {
        Self::new(value, true)
    }

    #[inline]
    pub fn new_none() -> Self
    where
        T: Default,
    {
        Self::new(T::default(), false)
    }

    #[inline]
    pub fn unwrap(self) -> T {
        assert!(self.is_some);

        self.value
    }

    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        T::cnd_select(&default, &self.value, self.is_some)
    }

    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.unwrap_or(f())
    }

    #[inline]
    #[allow(clippy::or_fun_call)]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        self.unwrap_or(T::default())
    }

    #[inline]
    pub fn unwrap_unchecked(self) -> T {
        self.value
    }

    #[inline]
    pub fn take(&mut self) -> Self {
        let ret = self.clone();
        self.is_some = false;
        ret
    }

    #[inline]
    pub fn replace(&mut self, value: T) -> Self {
        mem::replace(self, Self::new_some(value))
    }

    #[inline]
    pub fn insert(&mut self, value: T) -> &mut T {
        *self = Self::new_some(value);
        &mut self.value
    }

    #[inline]
    pub fn get_or_insert(&mut self, value: T) -> &mut T {
        self.value.cnd_assign(&value, self.is_none());
        self.is_some = true;
        &mut self.value
    }

    #[inline]
    pub fn get_or_set(&mut self, other: Self) -> &mut T {
        self.value = T::cnd_select(&other.value, &self.value, self.is_some);
        self.is_some |= other.is_some;
        &mut self.value
    }

    #[inline]
    pub fn force_some(&mut self) -> &mut T {
        self.is_some = true;
        &mut self.value
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        self.is_some
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> CndOption<U>
    where
        T: Default,
        U: CMov,
        F: FnOnce(T) -> U,
    {
        CndOption::new(
            f(T::cnd_select(&T::default(), &self.value, self.is_some)),
            self.is_some,
        )
    }

    #[inline]
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        T: Default,
        U: CMov,
        F: FnOnce(T) -> U,
    {
        self.map(f).unwrap_or(default)
    }

    #[inline]
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        T: Default,
        U: CMov,
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        self.map(f).unwrap_or_else(default)
    }

    #[inline]
    pub fn and<U>(self, other: CndOption<U>) -> CndOption<U>
    where
        U: CMov + Default,
    {
        CndOption::new(
            U::cnd_select(&U::default(), &other.value, self.is_some),
            self.is_some && other.is_some,
        )
    }

    #[inline]
    pub fn and_then<U, F>(self, f: F) -> CndOption<U>
    where
        T: Default,
        U: CMov,
        F: FnOnce(T) -> CndOption<U>,
    {
        let mut tmp = f(self.value);
        tmp.is_some &= self.is_some;
        tmp
    }

    #[inline]
    pub fn or(self, other: CndOption<T>) -> CndOption<T> {
        CndOption::new(
            T::cnd_select(&other.value, &self.value, self.is_some),
            self.is_some || other.is_some,
        )
    }

    #[inline]
    pub fn or_else<F>(self, f: F) -> CndOption<T>
    where
        F: FnOnce() -> CndOption<T>,
    {
        self.or(f())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Debug;

    fn assert_opt_eq<T: CMov + Debug + Eq>(a: CndOption<T>, b: Option<T>) {
        let a_opt: Option<T> = a.into();
        assert_eq!(a_opt, b);
    }

    #[allow(clippy::all)]
    fn test_cnd_option(opt_input: Option<u64>) {
        let cnd_input: CndOption<u64> = opt_input.clone().into();

        assert_eq!(cnd_input.is_some(), opt_input.is_some());
        assert_eq!(cnd_input.is_none(), opt_input.is_none());
        assert_eq!(
            cnd_input.clone().unwrap_or(1),
            opt_input.clone().unwrap_or(1)
        );
        assert_eq!(
            cnd_input.clone().unwrap_or_else(|| 1),
            opt_input.clone().unwrap_or_else(|| 1),
        );

        let mut cnd_input2 = cnd_input.clone();
        let mut opt_input2 = opt_input.clone();
        assert_opt_eq(cnd_input2.take(), opt_input2.take());
        assert_opt_eq(cnd_input2, opt_input2);

        let mut cnd_input3 = cnd_input.clone();
        let mut opt_input3 = opt_input.clone();
        assert_opt_eq(cnd_input3.replace(2), opt_input3.replace(2));
        assert_opt_eq(cnd_input3, opt_input3);

        let mut cnd_input4 = cnd_input.clone();
        let mut opt_input4 = opt_input.clone();
        assert_eq!(cnd_input4.insert(2), opt_input4.insert(2));
        assert_opt_eq(cnd_input4, opt_input4);

        let mut cnd_input5 = cnd_input.clone();
        let mut opt_input5 = opt_input.clone();
        assert_eq!(cnd_input5.get_or_insert(2), opt_input5.get_or_insert(2));
        assert_opt_eq(cnd_input5, opt_input5);

        assert_opt_eq(
            cnd_input.clone().map(|v| v + 1),
            opt_input.clone().map(|v| v + 1),
        );
        assert_eq!(
            cnd_input.clone().map_or(1, |v| v + 1),
            opt_input.clone().map_or(1, |v| v + 1),
        );
        assert_eq!(
            cnd_input.clone().map_or_else(|| 1, |v| v + 1),
            opt_input.clone().map_or_else(|| 1, |v| v + 1),
        );

        assert_opt_eq(
            cnd_input.clone().and(CndOption::new_some(1)),
            opt_input.clone().and(Some(1)),
        );
        assert_opt_eq::<u64>(
            cnd_input.clone().and(CndOption::new_none()),
            opt_input.clone().and(None),
        );

        assert_opt_eq(
            cnd_input.clone().and_then(|v| CndOption::new_some(v + 1)),
            opt_input.clone().and_then(|v| Some(v + 1)),
        );
        assert_opt_eq::<u64>(
            cnd_input.clone().and_then(|_| CndOption::new_none()),
            opt_input.clone().and_then(|_| None),
        );

        assert_opt_eq(
            cnd_input.clone().or(CndOption::new_some(2)),
            opt_input.clone().or(Some(2)),
        );
        assert_opt_eq(
            cnd_input.clone().or(CndOption::new_none()),
            opt_input.clone().or(None),
        );

        assert_opt_eq(
            cnd_input.clone().or_else(|| CndOption::new_some(2)),
            opt_input.clone().or_else(|| Some(2)),
        );
        assert_opt_eq(
            cnd_input.clone().or_else(|| CndOption::new_none()),
            opt_input.clone().or_else(|| None),
        );
    }

    #[test]
    fn test() {
        test_cnd_option(Some(1));
        test_cnd_option(None);
    }
}
