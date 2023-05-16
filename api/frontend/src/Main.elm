module Main exposing (..)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Encode as Encode
import Json.Decode as Decode

main: Program () Model Msg
main =
  Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view 
    }


-- MODEL
type alias Model =
  { form: Form
  , message: String
  , informations : List Information
  , selectedInformation : Maybe Information
  , isPopupOpen : Bool
  }

type alias Form =
  { url : String
  , tag : String
  , title : String
  }

type alias Information =
    { id : Int
    , url : String
    , tag : String
    , title : String
    }

type MyError
    = EmptyString

init : () -> (Model, Cmd Msg)
init _ =
  let
    model =
      { form = initForm
      , message = ""
      , informations = [] 
      , selectedInformation = Nothing
      , isPopupOpen = False 
      }
  in
  ( model, getInformation )
  

initForm : Form
initForm = 
  { url = "http;//"
  , tag = ""
  , title = ""
  }

type Msg
  = UpdateUrl String
  | UpdateTag String
  | UpdateTitle String
  | Check
  | Submit
  | GetInfos
  | DeleteInfo Information
  | GotTest (Result Http.Error String)
  | GotId  (Result Http.Error String)
  | GotInfos (Result Http.Error (List Information))
  | EditInfo Information
  | CancelEdit


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    UpdateUrl url ->
      let
        form =
          model.form

      in
      ( { model | form = { form | url = url } }, Cmd.none )

    UpdateTag tag ->
      let
        form =
          model.form

      in
      ( { model | form = { form | tag = tag } }, Cmd.none )

    UpdateTitle title ->
      let
        form =
          model.form          
      in
      ( { model | form = { form | title = title } }, Cmd.none )

    Check ->
      ( model, getMessage )

    Submit ->
      ( model, postInforation model )
    
    GetInfos ->
      ( model, getInformation )

    DeleteInfo information ->
      ( model, deleteInformation information )

    GotTest result ->
      case result of
        Ok message ->
          ( { model | form = initForm, message = message }, Cmd.none )

        Err _ ->
          ( model, Cmd.none )
  
    GotId result ->
      case result of
        Ok message ->
          ( { model | form = initForm, message = message }, getInformation )
        
        Err _ ->
          ( model, Cmd.none )

    GotInfos result ->
      case result of
        Ok informations ->
          ( { model | informations = informations }, Cmd.none )

        Err _ ->
          ( model, Cmd.none )
    
    EditInfo information ->
      ( { model | selectedInformation = Just information, isPopupOpen = True }, Cmd.none )

    CancelEdit ->
      ( { model | isPopupOpen = False }, Cmd.none )


subscriptions : Model -> Sub Msg
subscriptions _ =
  Sub.none


view : Model -> Html Msg
view model =
  div [] 
    [ div [ class "card-group" ] 
      ( List.map (viewCard model) model.informations 

      )
    , div [ class "input-block" ]
        [ viewInput "url" "URL" model.form.url UpdateUrl 
        , br [] []
        , viewInput "tag" "Tag" model.form.tag UpdateTag
        , br [] []
        , viewInput "title" "Title" model.form.title UpdateTitle
        ]
    , div [ class "button-block" ] 
        [ button [ onClick Check ] [ text "Check"]
        , button [ onClick Submit ] [ text "Submit"]
        , br [] []
      ]
    , if model.isPopupOpen then
        case model.selectedInformation of
          Just selected ->
            div [ class "edit-card" ]
                [ div [] [ text selected.title ]
                , br [] []
                , div [] [ text selected.tag ]
                , br [] []
                , button [ onClick CancelEdit ] [ text "編集完了" ] 
                , br [] []
                , button [ onClick CancelEdit ] [ text "キャンセル" ] ]
          Nothing ->
            text ""
      else
          text ""
    ]
    
viewCard : Model -> Information -> Html Msg
viewCard model information =
    div [ class "card" ] 
        [ div [ class "card__imgframe" ]
            [ img [ src "uopeople-logo.png", class "card__img", width 50, height 25 ] []
            ]
        , div [ class "card__textbox" ] 
            [ div [ class "card__titletext" ] [ text information.title ]
            , div [ class "card__overviewtext" ] [ text information.tag ]
            , button [ onClick (EditInfo information) ] [ text "編集" ]
            , button [ onClick (DeleteInfo information) ] [ text "削除" ]
            , a [ href information.url ] [ text "記事を見る" ]
            ]
        ]
    
viewInput : String -> String -> String -> (String -> msg) -> Html msg
viewInput t p v toMsg =
  input [ type_ t, placeholder p, value v, onInput toMsg, style "margin-bottom" "10px"] []

getMessage : Cmd Msg
getMessage =
  Http.request
    { method = "GET"
    , headers = 
      [ Http.header "Access-Control-Allow-Origin" "*"
      , Http.header "Content-Type" "application/json"
      ]
    , url = "http://localhost:8080/healthcheck"
    , body = Http.emptyBody
    , expect = Http.expectString GotTest
    , timeout = Nothing
    , tracker = Nothing
    }

postInforation : Model -> Cmd Msg
postInforation model =
  let 
    json = formJson model
  
  in
  Http.post
    {
      url = "http://localhost:8080/api/v1/info/create"
    , body = Http.jsonBody json
    , expect = Http.expectString GotId
    }

getInformation : Cmd Msg
getInformation =
  Http.request
    { method = "GET"
    , headers = 
      [ Http.header "Access-Control-Allow-Origin" "*"
      , Http.header "Content-Type" "application/json"
      ]
    , url = "http://localhost:8080/api/v1/info/all"
    , body = Http.emptyBody
    , expect = Http.expectJson GotInfos infosDecoder
    , timeout = Nothing
    , tracker = Nothing
    }

deleteInformation : Information -> Cmd Msg
deleteInformation information =
  let
    json = deleteJson information

  in
  Http.request
    { method = "POST"
    , headers = 
      [ Http.header "Access-Control-Allow-Origin" "*"
      ]
    , url = "http://localhost:8080/api/v1/info/delete"
    , body = Http.jsonBody json
    , expect = Http.expectString GotId
    , timeout = Nothing
    , tracker = Nothing
    }

deleteJson : Information -> Encode.Value
deleteJson information =
  Encode.object
    [ ( "id", Encode.int information.id )
    ]

formJson : Model -> Encode.Value
formJson model =
  Encode.object
    [ ( "url", Encode.string model.form.url )
    , ( "tag", Encode.string model.form.tag )
    , ( "title", Encode.string model.form.title )
    ]

infoDecoder : Decode.Decoder Information
infoDecoder =
  Decode.map4 Information
    (Decode.field "id" Decode.int)
    (Decode.field "url" Decode.string)
    (Decode.field "tag" Decode.string)
    (Decode.field "title" Decode.string)

infosDecoder : Decode.Decoder (List Information)
infosDecoder =
  Decode.list infoDecoder
