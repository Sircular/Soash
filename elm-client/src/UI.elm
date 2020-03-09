module UI exposing (cond, formField, cardPage, button, helpText, nestDivs)

import Html exposing (Attribute, Html, text, input, div, label, p, h1, a)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput, onClick)

cond : Html a -> Bool -> Html a
cond html show =
  if show then html else text ""

fromMaybe : Maybe (Html a) -> Html a
fromMaybe html =
  case html of
    Just a -> a
    Nothing -> text ""

type alias FormFieldOptions msg =
  { label : String
  , value : String
  , valid : Bool
  , inputType : String
  , disabled : Bool
  , helpText : String
  , events : List (Attribute msg) }

formField : FormFieldOptions a -> Html a
formField options =
  let showInvalid = (not options.valid) && (not options.disabled)
  in
    div [ class "field" ]
    [ label [ class "label" ] [text options.label]
    , div [ class "control" ]
      [ input
        ( [ value options.value
          , type_ options.inputType
          , disabled options.disabled
          , classList [("input", True), ("is-danger", showInvalid)] ]
          ++ options.events
          )
        []
      , cond (p [ class "help", class "is-danger" ]
        [text options.helpText]) showInvalid] ]

cardPage : Maybe String -> List (Html a) -> Html a
cardPage title contents =
  let
    cardHeader cardTitle =
      div [ class "card-header" ]
        [ h1 [ class "card-header-title", class "title" ] [ text cardTitle ] ]
  in
    div [ class "page-container" ]
      [ div [ class "card" ]
        [ fromMaybe <| Maybe.map cardHeader title
        , div [ class "card-content" ] contents ] ]

type alias ButtonOptions msg =
  { variant : String
  , text : String
  , events : List (Attribute msg)
  , loading : Bool }

button : ButtonOptions a -> Html a
button options =
  a ( [ classList
    [ ("button", True)
    , ("is-" ++ options.variant, True)
    , ("is-loading", options.loading) ]
    ] ++ options.events
    )
    [ text options.text ]

helpText : String -> String -> Html a
helpText variant text_ =
  p [ class ("has-text-" ++ variant) ] [ text text_ ]

nestDivs : List String -> List (Html a) -> Html a
nestDivs classSets contents =
  case classSets of
    (c::[]) ->
      div [class c] contents
    (c::cs) ->
      div [class c] [nestDivs cs contents]
    [] -> div [] contents
