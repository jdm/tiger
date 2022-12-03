use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cell::RefCell, collections::HashMap, hash::Hash, ops::Deref};

static NEXT_SUBSCRIPTION_ID: AtomicUsize = AtomicUsize::new(0);

type BoxedCallback<'a, T> = Box<dyn FnMut(&T) -> Response + 'a + Send>;
type SubscriptionId = usize;

#[derive(Debug)]
pub struct Observable<'o, T> {
    value: T,
    delegate: Delegate<'o, T>,
}

#[derive(Default)]
pub struct Delegate<'d, T> {
    subscriptions: RefCell<HashMap<SubscriptionId, BoxedCallback<'d, T>>>,
}

#[derive(Eq, Hash, PartialEq)]
pub struct Subscription {
    id: SubscriptionId,
}

pub enum Response {
    StaySubscribed,
    CancelSubscription,
}

impl<'d, T> Delegate<'d, T> {
    fn new() -> Self {
        Self {
            subscriptions: RefCell::new(HashMap::new()),
        }
    }

    pub fn subscribe<C: FnMut(&T) -> Response + 'd + Send>(&self, callback: C) -> Subscription {
        let id = NEXT_SUBSCRIPTION_ID.fetch_add(1, Ordering::SeqCst);
        let subscription = Subscription { id };
        self.subscriptions
            .borrow_mut()
            .insert(subscription.id, Box::new(callback));
        subscription
    }

    pub fn unsubscribe(&self, subscription: &Subscription) {
        self.subscriptions.borrow_mut().remove(&subscription.id);
    }

    pub fn broadcast(&self, value: &T) {
        let subscriptions_to_notify = self
            .subscriptions
            .borrow()
            .keys()
            .copied()
            .collect::<Vec<_>>();
        for subscription in subscriptions_to_notify {
            let (_, mut callback) = self
                .subscriptions
                .borrow_mut()
                .remove_entry(&subscription)
                .unwrap();
            match callback(value) {
                Response::CancelSubscription => (),
                Response::StaySubscribed => {
                    self.subscriptions
                        .borrow_mut()
                        .insert(subscription, callback);
                }
            };
        }
    }
}

impl<'o, T> Observable<'o, T> {
    fn new(value: T) -> Self {
        Self {
            value,
            delegate: Delegate {
                subscriptions: Default::default(),
            },
        }
    }

    pub fn subscribe<C: FnMut(&T) -> Response + 'o + Send>(&self, callback: C) -> Subscription {
        self.delegate.subscribe(callback)
    }

    pub fn unsubscribe(&self, subscription: &Subscription) {
        self.delegate.unsubscribe(subscription);
    }

    pub fn delegate(&self) -> &Delegate<'o, T> {
        &self.delegate
    }

    pub fn mutate<M>(&mut self, mutation: M)
    where
        M: FnOnce(&mut T),
    {
        mutation(&mut self.value);
        self.delegate.broadcast(&self.value);
    }
}

impl Delegate<'_, ()> {
    pub fn notify(&self) {
        self.broadcast(&());
    }
}

impl<T> Debug for Delegate<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Delegate")
            .field(
                "subscriptions",
                &format_args!("{} active subscriptions", self.subscriptions.borrow().len()),
            )
            .finish()
    }
}

impl<T> Default for Observable<'_, T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> Deref for Observable<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod test {

    use parking_lot::ReentrantMutex;
    use std::{cell::RefCell, ops::Deref, sync::Arc};

    use super::*;

    #[test]
    fn delegate_can_notify_subscribers() {
        let mut call_count = 0;
        {
            let d = Delegate::new();
            d.subscribe(|_| {
                call_count += 1;
                Response::StaySubscribed
            });
            d.notify();
            d.notify();
            d.notify();
        }
        assert_eq!(call_count, 3);
    }

    #[test]
    fn delegate_no_longer_notifies_after_unsubscribe() {
        let mut call_count = 0;
        {
            let d = Delegate::new();
            let subscription = d.subscribe(|_| {
                call_count += 1;
                Response::StaySubscribed
            });
            d.unsubscribe(&subscription);
            d.notify();
        }
        assert_eq!(call_count, 0);
    }

    #[test]
    fn cannot_unsubscribe_using_subscription_from_a_different_delegate() {
        let mut call_count = 0;
        {
            let d1 = Delegate::<()>::new();
            let d2 = Delegate::<()>::new();
            let _s1 = d1.subscribe(|_| {
                call_count += 1;
                Response::StaySubscribed
            });
            let s2 = d2.subscribe(|_| Response::StaySubscribed);
            d1.unsubscribe(&s2);
            d1.notify();
        }
        assert_eq!(call_count, 1);
    }

    #[test]
    fn unsubscribe_within_callback_is_noop() {
        let d = Arc::new(ReentrantMutex::new(Delegate::new()));
        let call_count = Arc::new(ReentrantMutex::new(RefCell::new(0)));
        let subscription = Arc::new(ReentrantMutex::new(RefCell::new(None)));

        let d_clone = d.clone();
        let call_count_clone = call_count.clone();
        let subscription_clone = subscription.clone();

        subscription
            .lock()
            .replace(Some(d.lock().subscribe(move |_| {
                let old_count = *call_count_clone.lock().borrow();
                *call_count_clone.lock().borrow_mut() = old_count + 1;
                d_clone
                    .lock()
                    .unsubscribe(subscription_clone.lock().deref().borrow().as_ref().unwrap());
                Response::StaySubscribed
            })));

        d.lock().notify();
        d.lock().notify();
        assert_eq!(*call_count.lock().borrow(), 2);
    }

    #[test]
    fn can_unsubscribe_using_callback_return_value() {
        let mut call_count = 0;
        {
            let d = Delegate::new();
            d.subscribe(|_| {
                call_count += 1;
                Response::CancelSubscription
            });
            d.notify();
            d.notify();
        }
        assert_eq!(call_count, 1);
    }

    #[test]
    fn observable_notifies_after_mutation() {
        let mut call_count = 0;
        {
            let mut o = Observable::new(0);
            o.subscribe(|_| {
                call_count += 1;
                Response::StaySubscribed
            });
            o.mutate(|value| *value = 42);
        }
        assert_eq!(call_count, 1);
    }

    #[test]
    fn observable_no_longer_notifies_after_unsubscribe() {
        let mut call_count = 0;
        {
            let mut o = Observable::new(0);
            let s = o.subscribe(|_| {
                call_count += 1;
                Response::StaySubscribed
            });
            o.mutate(|value| *value = 42);
            o.unsubscribe(&s);
            o.mutate(|value| *value = 43);
        }
        assert_eq!(call_count, 1);
    }
}
