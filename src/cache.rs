use std::{
    collections::HashMap,
    mem,
    sync::{
        atomic::{AtomicU8, Ordering},
        RwLock,
    },
    time::{Duration, Instant},
};

const MAX_OPS_BEFORE_PRUNE: u8 = 16;

pub struct TtlCache<T>
where T: Clone
{
    expiry: Duration,
    users: RwLock<HashMap<String, TtlEntry<T>>>,
    elapsed_ops: AtomicU8,
}

struct TtlEntry<T>
where T: Clone
{
    value: T,
    created: Instant,
}

impl<T> TtlCache<T>
where T: Clone
{
    pub fn new(expiry: Duration) -> Self {
        TtlCache {
            users: RwLock::new(HashMap::new()),
            elapsed_ops: AtomicU8::new(0),
            expiry,
        }
    }

    pub fn get_expiry(&self) -> Duration {
        self.expiry
    }

    pub fn insert(&self, key: &str, value: T) {
        let mut users = self.users.write().unwrap();
        users.insert(
            String::from(key),
            TtlEntry {
                value,
                created: Instant::now(),
            },
        );
        mem::drop(users);  // Necessary for the RwLock

        self.prune_old_entries(false);
    }

    pub fn get(&self, key: &str) -> Option<T> {
        let (value, _) = self.get_with_time(key)?;
        Some(value)
    }

    pub fn get_with_time(&self, key: &str) -> Option<(T, Instant)> {
        let users = self.users.read().unwrap();
        let entry = users.get(key)?;
        let computed = if entry.created.elapsed() > self.expiry {
            None
        } else {
            Some((entry.value.clone(), entry.created))
        };
        mem::drop(users);  // Necessary for the RwLock

        if let None = computed {
            self.prune_old_entries(true);
        } else {
            self.prune_old_entries(false);
        }
        computed
    }

    pub fn remove(&self, key: &str) {
        {
            let mut users = self.users.write().unwrap();
            users.remove(key);
        }
        self.prune_old_entries(false);
    }

    #[inline]
    fn prune_old_entries(&self, force: bool) {
        let mut users = self.users.write().unwrap();
        self.elapsed_ops.fetch_add(1, Ordering::Relaxed);
        if self.elapsed_ops.load(Ordering::Relaxed) >= MAX_OPS_BEFORE_PRUNE || force {
            let now = Instant::now();
            users
                .retain(|_, v| now.saturating_duration_since(v.created) < self.expiry);
            self.elapsed_ops.store(0, Ordering::Release);
        }
    }
}

