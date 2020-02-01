pub mod auth {
    use crate::{
        auth::{self, AuthStore, AuthenticatedUser},
        cache::TtlCache,
        constants,
    };
    use rocket::{
        http::{Cookie, Cookies, Status},
        request::Form,
        Route, State,
    };

    #[derive(FromForm)]
    pub struct LoginForm {
        pub username: String,
        pub password: String,
    }

    #[derive(FromForm)]
    pub struct RegisterForm {
        pub username: String,
        pub password: String,
    }

    #[post("/login", data = "<form>")]
    pub fn login(
        mut cookies: Cookies,
        auth_store: State<AuthStore>,
        auth_cache: State<TtlCache<AuthenticatedUser>>,
        form: Form<LoginForm>,
    ) -> Status {
        let result = auth_store.authenticate_user(&form.username, &form.password);
        let user = match result {
            Ok(user) => user,
            Err(_) => return Status::Unauthorized,
        };

        let token = auth::generate_session_token();
        auth_cache.insert(
            &token,
            AuthenticatedUser {
                id: user.id,
                name: user.name,
                token: token.clone(),
            },
        );

        let auth_cookie = Cookie::build(constants::SESSION_COOKIE_NAME, token)
            .path("/")
            .finish();

        cookies.add(auth_cookie);

        Status::Ok
    }

    #[post("/register", data = "<form>")]
    pub fn register(auth_store: State<AuthStore>, form: Form<RegisterForm>) -> Status {
        match auth_store.register_user(&form.username, &form.password) {
            Ok(_) => Status::Ok,
            Err(_) => Status::Unauthorized,
        }
    }

    #[get("/verify")]
    pub fn verify(_user: AuthenticatedUser) -> Status {
        Status::Ok
    }

    #[get("/logout")]
    pub fn logout(
        auth_cache: State<TtlCache<AuthenticatedUser>>,
        user: AuthenticatedUser,
        mut cookies: Cookies,
    ) {
        auth_cache.remove(&user.name);

        let auth_cookie = Cookie::build(constants::SESSION_COOKIE_NAME, "")
            .path("/")
            .finish();

        cookies.remove(auth_cookie);
    }

    pub fn routes() -> Vec<Route> {
        routes![login, register, verify, logout]
    }
}

pub mod note {
    use crate::{
        auth::AuthenticatedUser,
        search::{DocumentId, Note, NoteStore},
    };
    use rocket::{
        http::Status,
        response::status::{Accepted, Custom, NotFound},
        Route, State,
    };
    use rocket_contrib::json::Json;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct NewNote {
        title: String,
        body: String,
    }

    #[post("/new", format = "json", data = "<note>")]
    pub fn new(
        note_store: State<NoteStore>,
        user: AuthenticatedUser,
        note: Json<NewNote>,
    ) -> Result<Accepted<String>, Custom<String>> {
        let note = Note {
            id: 0,
            title: note.title.clone(),
            body: note.body.clone(),
        };
        match note_store.add_note(user.id, note) {
            Ok(id) => Ok(Accepted(Some(format!("{}", id)))),
            Err(_) => Err(Custom(
                Status::InternalServerError,
                String::from("Could not save note"),
            )),
        }
    }

    #[get("/<id>")]
    pub fn get(
        note_store: State<NoteStore>,
        user: AuthenticatedUser,
        id: DocumentId,
    ) -> Result<Json<Note>, NotFound<()>> {
        match note_store.get_note(user.id, id) {
            Ok(note) => Ok(Json(note)),
            Err(_) => Err(NotFound(())),
        }
    }

    #[delete("/<id>")]
    pub fn delete(note_store: State<NoteStore>, user: AuthenticatedUser, id: DocumentId) -> Status {
        match note_store.delete_note(user.id, id) {
            Ok(_) => Status::Ok,
            Err(_) => Status::NotFound,
        }
    }

    #[get("/<id>/similar?<count>")]
    pub fn similar(
        note_store: State<NoteStore>,
        user: AuthenticatedUser,
        id: DocumentId,
        count: Option<usize>,
    ) -> Result<Json<Vec<Note>>, Custom<String>> {
        let count = count.unwrap_or(10);
        match note_store.search_similar(user.id, id, count) {
            Ok(notes) => Ok(Json(notes)),
            Err(_) => Err(Custom(
                Status::InternalServerError,
                String::from("Could not search notes"),
            )),
        }
    }

    #[post("/<id>/update", format = "json", data = "<note>")]
    pub fn update(
        note_store: State<NoteStore>,
        user: AuthenticatedUser,
        id: DocumentId,
        note: Json<NewNote>,
    ) -> Result<Accepted<String>, Custom<String>> {
        let note = Note {
            id: 0,
            title: note.title.clone(),
            body: note.body.clone(),
        };
        match note_store.update_note(user.id, id, note) {
            Ok(id) => Ok(Accepted(Some(format!("{}", id)))),
            Err(_) => Err(Custom(
                Status::InternalServerError,
                String::from("Could not update note"),
            )),
        }
    }

    #[get("/search?<query>&<count>")]
    pub fn search(
        note_store: State<NoteStore>,
        user: AuthenticatedUser,
        query: String,
        count: Option<usize>,
    ) -> Result<Json<Vec<Note>>, Custom<String>> {
        let count = count.unwrap_or(10);
        match note_store.search_notes(user.id, &query, count) {
            Ok(notes) => Ok(Json(notes)),
            Err(_) => Err(Custom(
                Status::InternalServerError,
                String::from("Could not search notes"),
            )),
        }
    }

    pub fn routes() -> Vec<Route> {
        routes![new, get, update, delete, search, similar]
    }
}

pub mod static_files {
    use rocket::{http::{ContentType, Status}, response::content::Content, Route};
    use std::path::PathBuf;

    #[derive(RustEmbed)]
    #[folder = "$CARGO_MANIFEST_DIR/notable-client/dist"]
    struct Asset;

    #[get("/", rank = 0)]
    pub fn root() -> Result<Content<Vec<u8>>, Status> {
        static_file(None)
    }

    #[get("/<path..>", rank = 5)]
    pub fn static_file(path: Option<PathBuf>) -> Result<Content<Vec<u8>>, Status> {
        let path = match path {
            Some(path) => path,
            None => PathBuf::from("/"),
        };

        // exclude api endpoints
        if path.starts_with("api") || path.starts_with("/api") {
            return Err(Status::NotFound);
        }

        let mut content_type = match path.extension().and_then(|s| s.to_str()) {
            Some("html") => ContentType::HTML,
            Some("htm") => ContentType::HTML,
            Some("css") => ContentType::CSS,
            Some("js") => ContentType::JavaScript,
            _ => ContentType::Plain,
        };

        let file_data;
        if let Some(data) = Asset::get(path.to_str().unwrap()) {
            file_data = data;
        } else {
            // default to the index.html page for client-side routing
            content_type = ContentType::HTML;
            file_data = match Asset::get("index.html") {
                Some(data) => data,
                None => return Err(Status::InternalServerError),
            }
        }

        let file_data = file_data.into_owned();
        Ok(Content(content_type, file_data))
    }

    pub fn routes() -> Vec<Route> {
        routes![root, static_file]
    }
}
