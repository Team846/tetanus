use std::fmt::Debug;
use std::{marker::PhantomData, sync::Arc, sync::Mutex};

use crate::{consumer::Consumer, producer::Producer};

pub trait NodeReceiver {
    type In: Copy + Send;

    fn send(&self, msg: Self::In);
}

pub trait Node<'a>: NodeReceiver + Clone + Send {
    type Out: Copy + Send + 'a;

    fn chain<NewOut: Copy>(&mut self, other: impl Node<'a, In = Self::Out, Out = NewOut> + 'a);

    fn map<NewOut, F>(&mut self, f: F) -> MapNode<'a, Self::Out, NewOut, F>
    where
        NewOut: Copy + Send + 'a,
        F: Fn(Self::Out) -> NewOut + Clone + Send + 'a,
    {
        let node = MapNode {
            children: Arc::new(Mutex::new(Vec::new())),
            map_fn: f,
            in_: PhantomData,
            out: PhantomData,
        };

        self.chain(node.clone());

        node
    }

    fn zip<In2, Out2>(
        &mut self,
        mut other: impl Node<'a, In = In2, Out = Out2> + 'a,
    ) -> ZipNode<'a, Self::Out, Out2>
    where
        In2: Copy + Send + 'a,
        Out2: Copy + Send + 'a,
    {
        let node = ZipNode {
            children: Arc::new(Mutex::new(Vec::new())),
            current1: Arc::new(Mutex::new(None)),
            current2: Arc::new(Mutex::new(None)),
        };

        {
            let node2 = node.clone();
            self.map(move |msg1| {
                *node2.current1.lock().unwrap() = Some(msg1);
                node2.current2.lock().unwrap().map(|msg2| (msg1, msg2))
            })
            .chain(node.clone());
        }

        {
            let node2 = node.clone();
            other
                .map(move |msg2| {
                    *node2.current2.lock().unwrap() = Some(msg2);
                    node2.current1.lock().unwrap().map(|msg1| (msg1, msg2))
                })
                .chain(node.clone());
        }
        node
    }

    fn filter<F: Fn(Self::Out) -> bool + Clone + Send + 'a>(
        &mut self,
        predicate: F,
    ) -> FilterNode<'a, Self::Out, F> {
        let node = FilterNode {
            children: Arc::new(Mutex::new(Vec::new())),
            predicate,
        };

        self.chain(node.clone());
        node
    }

    fn produce<NewOut: Copy + Send + 'a, P: Producer<Msg = NewOut> + 'a>(
        &mut self,
        producer: Arc<Mutex<P>>,
    ) -> ProducerNode<'a, Self::Out, NewOut, P> {
        let node = ProducerNode {
            children: Arc::new(Mutex::new(Vec::new())),
            producer,
            in_: PhantomData,
        };

        self.chain(node.clone());
        node
    }

    fn consume<C: Consumer<Msg = Self::Out> + 'a>(
        &mut self,
        consumer: Arc<Mutex<C>>,
    ) -> ConsumerNode<'a, Self::Out, C> {
        let node = ConsumerNode {
            children: Arc::new(Mutex::new(Vec::new())),
            consumer,
        };

        self.chain(node.clone());
        node
    }

    fn log(&mut self) -> LoggingNode<'a, Self::Out>
    where
        Self::Out: Debug,
    {
        let node = LoggingNode {
            children: Arc::new(Mutex::new(Vec::new())),
        };

        self.chain(node.clone());
        node
    }
}

type NodeChildren<'a, T> = Arc<Mutex<Vec<Box<dyn NodeReceiver<In = T> + Send + 'a>>>>;

#[derive(Clone)]
pub struct BaseNode<'a, T: Copy> {
    children: NodeChildren<'a, T>,
}

impl<T: Copy> BaseNode<'_, T> {
    pub fn new() -> Self {
        BaseNode {
            children: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<'a, T: Copy + Send + 'a> NodeReceiver for BaseNode<'a, T> {
    type In = T;

    fn send(&self, msg: Self::In) {
        for child in self.children.lock().unwrap().iter() {
            child.send(msg);
        }
    }
}

impl<'a, T: Copy + Send + 'a> Node<'a> for BaseNode<'a, T> {
    type Out = T;

    fn chain<NewOut: Copy>(&mut self, other: impl Node<'a, In = Self::Out, Out = NewOut> + 'a) {
        self.children.lock().unwrap().push(Box::new(other));
    }
}

#[derive(Clone)]
pub struct MapNode<'a, In, Out, F>
where
    In: Copy,
    Out: Copy,
    F: Fn(In) -> Out,
{
    children: NodeChildren<'a, Out>,
    map_fn: F,
    in_: PhantomData<In>,
    out: PhantomData<Out>,
}

impl<'a, In, Out, F> NodeReceiver for MapNode<'a, In, Out, F>
where
    In: Copy + Send,
    Out: Copy + Send + 'a,
    F: Fn(In) -> Out + Clone + Send,
{
    type In = In;

    fn send(&self, msg: Self::In) {
        let processed = (self.map_fn)(msg);
        for child in self.children.lock().unwrap().iter() {
            child.send(processed);
        }
    }
}

impl<'a, In, Out, F> Node<'a> for MapNode<'a, In, Out, F>
where
    In: Copy + Send,
    Out: Copy + Send + 'a,
    F: Fn(In) -> Out + Send + Clone,
{
    type Out = Out;

    fn chain<NewOut: Copy>(&mut self, other: impl Node<'a, In = Self::Out, Out = NewOut> + 'a) {
        self.children.lock().unwrap().push(Box::new(other));
    }
}

#[derive(Clone)]
pub struct ZipNode<'a, In1: Copy, In2: Copy> {
    children: NodeChildren<'a, Option<(In1, In2)>>,
    current1: Arc<Mutex<Option<In1>>>,
    current2: Arc<Mutex<Option<In2>>>,
}

impl<'a, In1, In2> NodeReceiver for ZipNode<'a, In1, In2>
where
    In1: Copy + Send + 'a,
    In2: Copy + Send + 'a,
{
    type In = Option<(In1, In2)>;

    fn send(&self, msg: Self::In) {
        for child in self.children.lock().unwrap().iter() {
            child.send(msg);
        }
    }
}

impl<'a, In1, In2> Node<'a> for ZipNode<'a, In1, In2>
where
    In1: Copy + Send + 'a,
    In2: Copy + Send + 'a,
{
    type Out = Self::In;

    fn chain<NewOut: Copy>(&mut self, other: impl Node<'a, In = Self::Out, Out = NewOut> + 'a) {
        self.children.lock().unwrap().push(Box::new(other));
    }
}

#[derive(Clone)]
pub struct FilterNode<'a, T: Copy, F: Fn(T) -> bool> {
    children: NodeChildren<'a, T>,
    predicate: F,
}

impl<'a, T, F> NodeReceiver for FilterNode<'a, T, F>
where
    T: Copy + Send + 'a,
    F: Fn(T) -> bool + Clone + Send,
{
    type In = T;

    fn send(&self, msg: Self::In) {
        if (self.predicate)(msg) {
            for child in self.children.lock().unwrap().iter() {
                child.send(msg);
            }
        }
    }
}

impl<'a, T, F> Node<'a> for FilterNode<'a, T, F>
where
    T: Copy + Send + 'a,
    F: Fn(T) -> bool + Clone + Send,
{
    type Out = T;

    fn chain<NewOut: Copy>(&mut self, other: impl Node<'a, In = Self::Out, Out = NewOut> + 'a) {
        self.children.lock().unwrap().push(Box::new(other));
    }
}

pub struct ProducerNode<'a, In: Copy, Out: Copy, P: Producer<Msg = Out>> {
    children: NodeChildren<'a, Out>,
    producer: Arc<Mutex<P>>,
    in_: PhantomData<In>,
}

// #derive(Clone) doesn't work
impl<In: Copy, Out: Copy, P: Producer<Msg = Out>> Clone for ProducerNode<'_, In, Out, P> {
    fn clone(&self) -> Self {
        ProducerNode {
            children: self.children.clone(),
            producer: self.producer.clone(),
            in_: PhantomData,
        }
    }
}

impl<'a, In, Out, P> NodeReceiver for ProducerNode<'a, In, Out, P>
where
    In: Copy + Send + 'a,
    Out: Copy + Send + 'a,
    P: Producer<Msg = Out>,
{
    type In = In;

    fn send(&self, _msg: Self::In) {
        let msg = self.producer.lock().unwrap().next();
        for child in self.children.lock().unwrap().iter() {
            child.send(msg);
        }
    }
}

impl<'a, In, Out, P> Node<'a> for ProducerNode<'a, In, Out, P>
where
    In: Copy + Send + 'a,
    Out: Copy + Send + 'a,
    P: Producer<Msg = Out>,
{
    type Out = Out;

    fn chain<NewOut: Copy>(&mut self, other: impl Node<'a, In = Self::Out, Out = NewOut> + 'a) {
        self.children.lock().unwrap().push(Box::new(other));
    }
}

pub struct ConsumerNode<'a, T: Copy, C: Consumer<Msg = T>> {
    children: NodeChildren<'a, T>,
    consumer: Arc<Mutex<C>>,
}

// #derive(Clone) doesn't work
impl<T: Copy, C: Consumer<Msg = T>> Clone for ConsumerNode<'_, T, C> {
    fn clone(&self) -> Self {
        ConsumerNode {
            children: self.children.clone(),
            consumer: self.consumer.clone(),
        }
    }
}

impl<'a, T, C> NodeReceiver for ConsumerNode<'a, T, C>
where
    T: Copy + Send,
    C: Consumer<Msg = T>,
{
    type In = T;

    fn send(&self, msg: Self::In) {
        self.consumer.lock().unwrap().output(msg);
        for child in self.children.lock().unwrap().iter() {
            child.send(msg);
        }
    }
}

impl<'a, T, C> Node<'a> for ConsumerNode<'a, T, C>
where
    T: Copy + Send + 'a,
    C: Consumer<Msg = T>,
{
    type Out = T;

    fn chain<NewOut: Copy>(&mut self, other: impl Node<'a, In = Self::Out, Out = NewOut> + 'a) {
        self.children.lock().unwrap().push(Box::new(other));
    }
}

#[derive(Clone)]
pub struct LoggingNode<'a, T: Copy + Debug> {
    children: NodeChildren<'a, T>,
}

impl<'a, T> NodeReceiver for LoggingNode<'a, T>
where
    T: Copy + Debug + Send + 'a,
{
    type In = T;

    fn send(&self, msg: Self::In) {
        println!("{:?}", msg);
        for child in self.children.lock().unwrap().iter() {
            child.send(msg);
        }
    }
}

impl<'a, T> Node<'a> for LoggingNode<'a, T>
where
    T: Copy + Debug + Send + 'a,
{
    type Out = T;

    fn chain<NewOut: Copy>(&mut self, other: impl Node<'a, In = Self::Out, Out = NewOut> + 'a) {
        self.children.lock().unwrap().push(Box::new(other));
    }
}
