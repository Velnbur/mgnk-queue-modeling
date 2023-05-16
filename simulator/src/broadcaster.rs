use std::sync::mpsc::{RecvError, TryRecvError};

///! Broadcasts messsages receiver
pub(crate) struct Receiver<T> {
    receiver: std::sync::mpsc::Receiver<T>,
}

impl<T> Receiver<T> {
    ///! Tries to receive a message. If there is no message, returns None.
    pub(crate) fn try_recv(&self) -> Result<Option<T>, RecvError> {
        match self.receiver.try_recv() {
            Ok(message) => Ok(Some(message)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(RecvError),
        }
    }
}

///! Broadcasts messsages sender.
pub(crate) struct Sender<T: Clone> {
    senders: Vec<std::sync::mpsc::Sender<T>>,
}

impl<T: Clone> Sender<T> {
    ///! Create a new receiver to which all messages will be broadcasted.
    pub(crate) fn subscribe(&mut self) -> Receiver<T> {
        let (sender, receiver) = std::sync::mpsc::channel();
        self.senders.push(sender);
        Receiver { receiver }
    }

    pub(crate) fn send(&self, message: T) -> Result<(), RecvError> {
        for sender in &self.senders {
            if sender.send(message.clone()).is_err() {
                return Err(RecvError);
            }
        }
        Ok(())
    }
}

pub(crate) fn channel<T: Clone>() -> (Sender<T>, Receiver<T>) {
    let (sender, receiver) = std::sync::mpsc::channel();
    let sender = Sender {
        senders: vec![sender],
    };
    (sender, Receiver { receiver })
}
