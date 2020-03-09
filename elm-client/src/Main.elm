module Main exposing (..)

import Browser
import Browser.Navigation as Nav
import Context exposing (Context)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick)
import Page.Login as Login
import UI
import Url exposing (Url)

main : Program Flags Model Msg
main =
  Browser.application
  { init = init
  , update = update
  , view = view
  , onUrlChange = UrlChange
  , onUrlRequest = LinkClicked
  , subscriptions = subscriptions }

type alias Flags = ()

type alias Model =
  { context : Context
  , pageModel : PageModel
  , navbarExpanded : Bool
  }

type PageModel
  = LoginModel Login.Model

type Msg
  = Noop ()
  | UrlChange Url
  | LinkClicked Browser.UrlRequest
  | LoginMsg Login.Msg
  | ToggleNavbar

init : Flags -> Url -> Nav.Key -> ( Model, Cmd Msg )
init flags url navkey =
  let
      firstContext = Context.init flags
      (context, loginModel, loginCmd) = Login.init firstContext
  in
      ( { context = context
        , pageModel = LoginModel loginModel
        , navbarExpanded = False }
      , Cmd.map LoginMsg loginCmd )

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
  case (msg, model.pageModel) of
    (LoginMsg subMsg, LoginModel subModel) ->
      updatePage LoginModel LoginMsg model <|
        Login.update subMsg (model.context, subModel)
    (ToggleNavbar, _) ->
      ({ model | navbarExpanded = not model.navbarExpanded }, Cmd.none)
    _ -> (model, Cmd.none)

updatePage : (subModel -> PageModel) -> (subMsg -> Msg) -> Model ->
 (subModel, Maybe Context.Msg, Cmd subMsg) -> (Model, Cmd Msg)
updatePage mapModel mapMsg model (subModel, contextMsg, subCmd) =
  let
      (context, contextCmd) =
        case contextMsg of
          Just msg -> Context.update msg model.context
          Nothing -> (model.context, Cmd.none)
  in
      ( { model
        | pageModel = mapModel subModel
        , context = context }
      , Cmd.batch [Cmd.map mapMsg subCmd, Cmd.map Noop contextCmd ] )

view : Model -> Browser.Document Msg
view model =
  let
      (pageTitle, pageBody) = case model.pageModel of
        (LoginModel subModel) ->
          Login.view (model.context, subModel)
  in
    { title = case pageTitle of
      Just title -> title ++ " - Soash"
      Nothing -> "Soash"
    , body =
      [ viewNavbar model
      , div
        [ class "pageContainer" ]
        [ Html.map LoginMsg pageBody ]
      ]
    }

viewNavbar : Model -> Html Msg
viewNavbar model =
  UI.nestDivs
  [ "navbar navbar-floating has-shadow has-background-white-ter"
  , "container" ]
  [ viewNavbarBrand model
  , viewNavbarMenu model
    [ ("Login", "/login")
    , ("Register", "/register")
    ]
  ]

viewNavbarBrand : Model -> Html Msg
viewNavbarBrand model =
  div
  [ class "navbar-brand" ]
  [ div
    [ class "navbar-item" ]
    [ img [ src "img/logo-dark.svg" ] []
  ]
  , a
    [ class "navbar-burger burger"
    , classList [("is-active", model.navbarExpanded)]
    , onClick ToggleNavbar ]
    [ span [] [], span [] [], span [] [] ]
  ]

viewNavbarMenu : Model -> List (String, String) -> Html Msg
viewNavbarMenu model pages =
  div
  [ class "navbar-menu", classList [("is-active", model.navbarExpanded)] ]
  [ div
    [ class "navbar-start" ]
    <| List.map (viewNavbarLink model) pages ]

viewNavbarLink : Model -> (String, String) -> Html Msg
viewNavbarLink model (title, target) =
  a
  [ href target, class "navbar-item" ]
  [ text title ]

subscriptions : Model -> Sub Msg
subscriptions model = Sub.none
