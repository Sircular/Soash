use atomic_counter::{AtomicCounter, RelaxedCounter};
use bcrypt::BcryptError;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Status,
    request::{FromRequest, Request, State},
    response::Response,
    Outcome,
};
use serde::{Deserialize, Serialize};
use std::sync::{PoisonError, RwLock};
use uuid::{adapter::Hyphenated, Uuid};

use crate::{cache::TtlCache, constants};

pub fn generate_session_token() -> String {
    format!("{}", Hyphenated::from(Uuid::new_v4()))
}

#[derive(Debug)]
pub enum AuthenticationError {
    UsernameTaken,
    UserNotFound,
    IncorrectPassword,
    HashError(BcryptError),
    StoreInaccessible,
}

impl<T> From<PoisonError<T>> for AuthenticationError {
    fn from(_error: PoisonError<T>) -> Self {
        Self::StoreInaccessible
    }
}

impl From<pickledb::error::Error> for AuthenticationError {
    fn from(_error: pickledb::error::Error) -> Self {
        Self::StoreInaccessible
    }
}

impl From<BcryptError> for AuthenticationError {
    fn from(error: BcryptError) -> Self {
        Self::HashError(error)
    }
}

// impl From<

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub password: String,
}

pub struct AuthStore {
    db: RwLock<PickleDb>,
    id_counter: RelaxedCounter,
}

impl AuthStore {
    pub fn new(db_path: &str) -> Self {
        let db = PickleDb::load(
            db_path,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        );

        let db = match db {
            Ok(db) => db,
            Err(_) => PickleDb::new(
                db_path,
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            ),
        };

        let next_id = db.total_keys();

        AuthStore {
            db: RwLock::new(db),
            id_counter: RelaxedCounter::new(next_id),
        }
    }

    pub fn register_user(&self, name: &str, password: &str) -> Result<(), AuthenticationError> {
        let mut db = self.db.write()?;
        let existing_user: Option<User> = db.get(&name);
        if let Some(_) = existing_user {
            return Err(AuthenticationError::UsernameTaken);
        }
        let id = self.id_counter.inc() as u64;
        let password = bcrypt::hash(&password, constants::BCRYPT_ITERATIONS)?;
        db.set(
            &name,
            &User {
                id,
                name: String::from(name),
                password: String::from(password),
            },
        )?;
        Ok(())
    }

    pub fn authenticate_user(
        &self,
        name: &str,
        password: &str,
    ) -> Result<User, AuthenticationError> {
        use AuthenticationError::*;
        let db = self.db.read()?;
        let stored_user = match db.get::<User>(name) {
            None => return Err(UserNotFound),
            Some(user) => user,
        };
        if bcrypt::verify(password, &stored_user.password)? {
            Ok(stored_user)
        } else {
            Err(IncorrectPassword)
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: u64,
    pub name: String,
    pub token: String,
}

#[derive(Debug)]
pub enum AuthTokenError {
    MissingToken,
    InvalidToken,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = AuthTokenError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        use AuthTokenError::*;

        let cookies = request.cookies();
        let session_cookie = cookies.get(constants::SESSION_COOKIE_NAME);
        if let Some(cookie_value) = session_cookie {
            let user_store = request
                .guard::<State<TtlCache<AuthenticatedUser>>>()
                .expect("user store not initialized");

            if let Some(user) = user_store.get(cookie_value.value()) {
                return Outcome::Success(user.clone());
            } else {
                return Outcome::Failure((Status::Unauthorized, InvalidToken));
            }
        } else {
            return Outcome::Failure((Status::Unauthorized, MissingToken));
        }
    }
}

pub struct TokenRefreshFairing {}

impl Fairing for TokenRefreshFairing {
    fn info(&self) -> Info {
        Info {
            name: "Token Refresh",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let cookies = request.cookies();
        let session_cookie = cookies.get(constants::SESSION_COOKIE_NAME);
        if let Some(cookie) = session_cookie {
            let user_store = request
                .guard::<State<TtlCache<AuthenticatedUser>>>()
                .expect("user store not initialized");
            if let Some((user, created)) = user_store.get_with_time(cookie.value()) {
                if created.elapsed() > user_store.get_expiry() / 2 {
                    let new_token = generate_session_token();
                    let mut new_cookie = cookie.clone();
                    new_cookie.set_value(new_token.clone());
                    response.set_header(new_cookie);
                    user_store.insert(&new_token, user.clone());
                }
            }
        }
    }
}
