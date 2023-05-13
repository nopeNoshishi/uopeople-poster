module Main exposing (..)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http

main: Program () Model Msg
main =
  Browser.element { init = init, update = update, subscriptions = subscriptions, view = view }

type alias Model =
  { form: Form,
    message: String
  }

type alias Form =
  { url : String
  , tag : String
  }


init : () -> (Model, Cmd Msg)
init _ =
  let
    model =
      { form = initForm, message = "" }
  in
  ( model, Cmd.none )
  

initForm : Form
initForm = 
  { url = ""
  , tag = ""
  }

type Msg
  = UpdateUrl String
  | UpdateTag String
  | Submit
  | GotTest (Result Http.Error String)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    UpdateUrl url ->
      let
        form =
          model.form

        newForm =
          { form | url = url }
        
        newModel = 
          { model | form = newForm }

      in
      ( newModel, Cmd.none )

    UpdateTag tag ->
      let
        form =
          model.form

        newForm =
          { form | tag = tag }

        newModel = 
          { model | form = newForm }

      in
      ( newModel, Cmd.none )

    Submit ->
      ( model, getMessage model )

    GotTest result ->
      case result of
        Ok message ->
          let
            newModel =
              { form = initForm, message = message}
          in
          ( newModel, Cmd.none )

        Err _ ->
          ( model, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions _ =
  Sub.none


view : Model -> Html Msg
view model =
  div [ style "display" "flex"
      , style "align-items" "center"
      , style "justify-content" "center"
      , style "height" "100vh"
      , style "flex-direction" "column"
    ]
    [ viewInput "url" "URL" model.form.url UpdateUrl 
    , viewInput "tag" "Tag" model.form.tag UpdateTag
    , button [ onClick Submit ] [ text "Submit"]
    , div [] [ text (model.message) ] 
    ]

viewInput : String -> String -> String -> (String -> msg) -> Html msg
viewInput t p v toMsg =
  input [ type_ t, placeholder p, value v, onInput toMsg, style "margin-bottom" "10px"] []


getMessage : Model -> Cmd Msg
getMessage model =
  Http.get
    { url = "http://localhost:8080/api/v1/healthcheck?" ++ model.form.tag
    , expect = Http.expectString GotTest
    }