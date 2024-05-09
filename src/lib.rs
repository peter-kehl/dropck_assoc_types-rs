// https://github.com/rust-lang/rfcs/blob/master/text/1238-nonparametric-dropck.md,
// https://github.com/rust-lang/rfcs/blob/master/text/1327-dropck-param-eyepatch.md and
// https://doc.rust-lang.org/nomicon/dropck.html#an-escape-hatchdon't seem to apply here.

pub trait Out<T> {}
//pub trait OutItemRef<'slice, T> {}

pub trait TransRef<'slice, T: 'slice + Clone> {
    type Own<'own>;

    type Out<'out>: Out<T> + 'out
    where
        T: 'out;

    type OutRef<'out>: OutItemRef<'slice, T> + 'out
    where
        T: 'out,
        'slice: 'out;

    fn reserve_own<'own>() -> Self::Own<'own>;
    fn reserve_out<'out>() -> Self::Out<'out>;

    /*fn ini_out_move_seed<'out>(out: &mut Self::Out<'out>, mut out_seed: Self::OutSeed)
    where
        Self::Out<'out>: 'out,
    {
        Self::ini_out_mut_seed(out, &mut out_seed);
    }
    fn ini_out_mut_seed<'out, 'outref>(
        out: &'outref mut Self::Out<'out>,
        out_seed: &'outref mut Self::OutSeed,
    ) where
        Self::Out<'out>: 'out;*/

    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        out: &mut Self::Out<'out>,
        own: &'ownref Self::Own<'own>,
    ) where
        'ownref: 'slice;

    /*fn with_trans_ref<'slice, T: 'slice + Clone, TR: 'slice + TransRef<'slice, T>>() {
    let own = TR::reserve_own();
    let mut out = TR::reserve_out();
    TR::set_out(&mut out, &own);*/
}

/// An indicator/holder/carrier of a type that implements [TransRef].
pub trait TransRefTypeIndicator<T: Clone> {
    type TransRefImpl<'slice>: TransRef<'slice, T>
    where
        T: 'slice;
}

fn with_trans_ref_holder<T: Clone, TRI: TransRefTypeIndicator<T>>() {
    let own =
        <<TRI as TransRefTypeIndicator<T>>::TransRefImpl<'_> as TransRef<'_, T>>::reserve_own();
    let mut out =
        <<TRI as TransRefTypeIndicator<T>>::TransRefImpl<'_> as TransRef<'_, T>>::reserve_out();
    <<TRI as TransRefTypeIndicator<T>>::TransRefImpl<'_> as TransRef<'_, T>>::set_out(
        &mut out, &own,
    );
}
