use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cell::RefCell, collections::HashMap, hash::Hash, ops::Deref};

static NEXT_SUBSCRIPTION_ID: AtomicUsize = AtomicUsize::new(0);

type BoxedCallback<'a, T> = Box<dyn FnMut(&T) -> Observer + 'a + Send>;
type SubscriptionId = usize;

pub struct Observable<'o, T> {
    value: T,
    subscriptions: RefCell<HashMap<SubscriptionId, BoxedCallback<'o, T>>>,
}

pub struct Delegate<'a, 'o, T> {
    observable: &'a Observable<'o, T>,
}

#[derive(Eq, Hash, PartialEq)]
pub struct Subscription {
    id: SubscriptionId,
}

pub enum Observer {
    StaySubscribed,
    CancelSubscription,
}

impl<'a, 'o, T> Delegate<'a, 'o, T> {
    pub fn subscribe<C: FnMut(&T) -> Observer + 'o + Send>(&self, callback: C) -> Subscription {
        self.observable.subscribe(callback)
    }

    pub fn unsubscribe(&self, subscription: &Subscription) {
        self.observable.unsubscribe(subscription);
    }
}

impl<'o, T> Observable<'o, T> {
    fn new(value: T) -> Self {
        Self {
            value,
            subscriptions: RefCell::new(HashMap::new()),
        }
    }

    pub fn delegate<'b>(&'b self) -> Delegate<'b, 'o, T> {
        Delegate { observable: self }
    }

    pub fn mutate<M>(&mut self, mutation: M)
    where
        M: FnOnce(&mut T),
    {
        mutation(&mut self.value);
        self.notify();
    }

    pub fn notify(&self) {
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
            match callback(&self.value) {
                Observer::CancelSubscription => (),
                Observer::StaySubscribed => {
                    self.subscriptions
                        .borrow_mut()
                        .insert(subscription, callback);
                }
            };
        }
    }

    pub fn subscribe<C: FnMut(&T) -> Observer + 'o + Send>(&self, callback: C) -> Subscription {
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
}

impl<T> Debug for Observable<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Observable")
            .field("value", &self.value)
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
    fn can_notify_subscribers() {
        let mut call_count = 0;
        {
            let o = Observable::new(0);
            o.subscribe(|_| {
                call_count += 1;
                Observer::StaySubscribed
            });
            o.notify();
            o.notify();
            o.notify();
        }
        assert_eq!(call_count, 3);
    }

    #[test]
    fn notifies_after_mutation() {
        let mut call_count = 0;
        {
            let mut o = Observable::new(0);
            o.subscribe(|_| {
                call_count += 1;
                Observer::StaySubscribed
            });
            o.mutate(|value| *value = 42);
        }
        assert_eq!(call_count, 1);
    }

    #[test]
    fn no_longer_notifies_after_unsubscribe() {
        let mut call_count = 0;
        {
            let mut o = Observable::new(0);
            let subscription = o.subscribe(|_| {
                call_count += 1;
                Observer::StaySubscribed
            });
            o.unsubscribe(&subscription);
            o.mutate(|value| *value = 42);
            o.notify();
        }
        assert_eq!(call_count, 0);
    }

    #[test]
    fn cannot_unsubscribe_using_subscription_from_a_different_observable() {
        let mut call_count = 0;
        {
            let o1 = Observable::new(0);
            let o2 = Observable::new(0);
            let _s1 = o1.subscribe(|_| {
                call_count += 1;
                Observer::StaySubscribed
            });
            let s2 = o2.subscribe(|_| Observer::StaySubscribed);
            o1.unsubscribe(&s2);
            o1.notify();
        }
        assert_eq!(call_count, 1);
    }

    #[test]
    fn unsubscribe_within_callback_is_noop() {
        let o = Arc::new(ReentrantMutex::new(Observable::new(0)));
        let call_count = Arc::new(ReentrantMutex::new(RefCell::new(0)));
        let subscription = Arc::new(ReentrantMutex::new(RefCell::new(None)));

        let o_clone = o.clone();
        let call_count_clone = call_count.clone();
        let subscription_clone = subscription.clone();

        subscription
            .lock()
            .replace(Some(o.lock().subscribe(move |_| {
                let old_count = *call_count_clone.lock().borrow();
                *call_count_clone.lock().borrow_mut() = old_count + 1;
                o_clone
                    .lock()
                    .unsubscribe(subscription_clone.lock().deref().borrow().as_ref().unwrap());
                Observer::StaySubscribed
            })));

        o.lock().notify();
        o.lock().notify();
        assert_eq!(*call_count.lock().borrow(), 2);
    }

    #[test]
    fn can_unsubscribe_using_callback_return_value() {
        let mut call_count = 0;
        {
            let mut o = Observable::new(0);
            o.subscribe(|_| {
                call_count += 1;
                Observer::CancelSubscription
            });
            o.mutate(|value| *value = 42);
            o.notify();
        }
        assert_eq!(call_count, 1);
    }
}
