module Page.Login exposing (Model, Msg, init, update, view)

import Context exposing (UserState, Context)
import Html exposing (Html, text)
import Html.Events exposing (onInput, onClick)
import Http
import UI

type LoginState
  = Unsubmitted
  | Invalid
  | Submitting
  | Success
  | Failure

type alias Model =
  { username : String
  , password : String
  , loginState : LoginState
  }

type Msg
  = ChangeUsername String
  | ChangePassword String
  | Submit
  | ReceiveLogin (Result Http.Error String)

init : Context -> (Context, Model, Cmd Msg)
init context =
  (context
  , { username = ""
    , password = ""
    , loginState = Unsubmitted
    }
  , Cmd.none)

update : Msg -> (Context, Model) -> (Model, Maybe Context.Msg, Cmd Msg)
update msg (context, model) =
  case msg of
    ChangeUsername username ->
      ({ model | username = username }, Nothing, Cmd.none)
    ChangePassword password ->
      ({ model | password = password }, Nothing, Cmd.none)
    Submit ->
      if model.password == "" || model.password == ""
      then
        ({ model | loginState = Invalid }
        , Nothing
        , Cmd.none)
      else
        ({ model | loginState = Submitting }
        , Nothing
        , submitLogin model.username model.password)
    ReceiveLogin result ->
      case result of
        Ok _ ->
          ({ model | loginState = Success }
          , Nothing
          , Cmd.none)
        Err _ ->
          ({ model | loginState = Failure, password = "" }
          , Nothing
          , Cmd.none)

submitLogin : String -> String -> Cmd msg
submitLogin username password = Cmd.none

view : (Context, Model) -> (Maybe String, Html Msg)
view ( context, model ) =
  ( Just "Log In"
  , UI.cardPage (Just "Log In")
    [ UI.formField
      { label = "Username"
      , value = model.username
      , valid = model.loginState == Unsubmitted || model.username /= ""
      , inputType = "text"
      , disabled = model.loginState == Submitting
      , helpText = "Please enter your username."
      , events = [ onInput ChangeUsername ]
      }
    , UI.formField
      { label = "Password"
      , value = model.password
      , valid = model.loginState == Unsubmitted || model.password /= ""
      , inputType = "password"
      , disabled = model.loginState == Submitting
      , helpText = "Please enter your password."
      , events = [ onInput ChangePassword ]
      }
    , UI.cond
      (UI.helpText "danger" "Your username or password was incorrect.")
      (model.loginState == Failure)
    , UI.button
      { variant = "primary"
      , text = "Log In"
      , events = [ onClick (Submit) ]
      , loading = model.loginState == Submitting
      }
    ])
