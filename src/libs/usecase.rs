pub trait UseCase<I: Input<O>, O: Output> {
    fn execute(&self, input: I) -> O;
}

pub trait Input<O: Output> {}

pub trait Output {}