module Context exposing (UserState, Context, Msg, init, update)

type Msg
  = LogIn String
  | LogOut

type alias Context =
  { userState : UserState
  }

type UserState
  = NotLoggedIn
  | LoggedIn String

init : () -> Context
init _ =
  { userState = NotLoggedIn
  }

update : Msg -> Context -> (Context, Cmd ())
update msg context =
  case msg of
    LogIn key ->
      ({ context | userState = LoggedIn key }, Cmd.none)
    LogOut ->
      ({ context | userState = NotLoggedIn }, Cmd.none)
