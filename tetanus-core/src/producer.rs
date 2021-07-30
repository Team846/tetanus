pub trait Producer: Send + Sync {
    type Msg: Copy;

    fn next(&self) -> Self::Msg;
}
