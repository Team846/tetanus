pub trait Consumer: Send + Sync {
    type Msg: Copy;

    fn output(&mut self, msg: Self::Msg);
}
