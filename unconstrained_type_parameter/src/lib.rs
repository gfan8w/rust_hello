
trait Invoke {
    type S;
    type E;

    fn fun(&mut self) -> Result<Self::S, Self::E>;
}

impl<F, S, E> Invoke for F
    where
        F: Fn() -> Result<S, E>,
{
    type S = S;
    type E = E;

    fn fun(&mut self) -> Result<S, E> {
        self()
    }
}


