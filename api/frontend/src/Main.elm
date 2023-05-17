module Main exposing (..)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Encode as Encode
import Json.Decode as Decode
import Toast
import Process
import Task


-- Main
main: Program () Model Msg
main =
  Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view 
    }


-- Model
type alias Model =
  { form: Form
  , editForm: Form
  , message: String
  , tray : Toast.Tray Toast 
  , problems : List Problem
  , editProblems : List Problem
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

type alias ResponseMessage =
  {  message : String }

type alias Toast =
    { message : String
    , color : Color
    }

type Color
    = Red
    | Blue
    | Green

type Problem
    = InvalidEntry ValidatedField String
    | ServerError String

init : () -> (Model, Cmd Msg)
init _ =
  let
    model =
      { form = initForm
      , editForm = initForm
      , message = ""
      , tray = Toast.tray
      , problems = []
      , editProblems = []
      , informations = [] 
      , selectedInformation = Nothing
      , isPopupOpen = False 
      }
  in
  ( model, 
    Cmd.batch
      [ getInformation
      ] 
  )
  
initForm : Form
initForm = 
  { url = "http;//"
  , tag = ""
  , title = ""
  }


-- Update Msg
type Msg
  = UpdateUrl String
  | UpdateTag String
  | UpdateTitle String
  | UpdateSelectedUrl String
  | UpdateSelectedTag String
  | UpdateSelectedTitle String
  | Submit
  | SubmitEdited Int
  | GetInfos
  | DeleteInfo Information
  | GotCreated  (Result Http.Error ResponseMessage)
  | GotUpdated (Result Http.Error ResponseMessage)
  | GotDeleted (Result Http.Error ResponseMessage)
  | GotInfos (Result Http.Error (List Information))
  | EditInfo Information
  | CancelEdit
  | ToastMsg Toast.Msg
  | AddToast Toast

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

    UpdateSelectedUrl url ->
      let
        editForm =
          model.editForm

      in
      ( { model | editForm = { editForm | url = url } }, Cmd.none )

    UpdateSelectedTag tag ->
      let
        editForm =
          model.editForm

      in
      ( { model | editForm = { editForm | tag = tag } }, Cmd.none )

    UpdateSelectedTitle title ->
      let
        editForm =
          model.editForm          
      in
      ( { model | editForm = { editForm | title = title } }, Cmd.none )

    Submit ->
      case validate model.form of
        Ok validForm ->
          ( { model | problems = [] }
          , postInforation validForm
          )

        Err problems ->
          ( { model | problems = problems }
          , Cmd.none
          )

    SubmitEdited info_id ->
      case validate model.editForm of
        Ok validForm ->
          ( { model | editProblems = [] }
          , putInforation info_id validForm
          )

        Err problems ->
          ( { model | editProblems = problems }
          , Cmd.none
          )

    GetInfos ->
      ( model, getInformation )

    DeleteInfo information ->
      ( model, deleteInformation information )
  
    GotCreated result ->
      case result of
        Ok response ->
          ( { model | form = initForm, message = response.message }
            , Cmd.batch 
            [ getInformation
            , delay 0 (AddToast { message = response.message, color = Green } )
            ] 
          )
        
        Err _ ->
          ( model, delay 0 (AddToast { message = "作成中にエラーが発生しました。", color = Red } ) )

    GotUpdated result ->
      case result of
        Ok response ->
          ( { model | message = response.message, isPopupOpen = False }
            , Cmd.batch 
            [ getInformation
            , delay 0 (AddToast { message = response.message, color = Green } )
            ] 
          )
        
        Err _ ->
          ( model, delay 0 (AddToast { message = "更新中ににエラーが発生しました。", color = Red } ) )

    GotDeleted result ->
      case result of
        Ok response ->
          ( { model | message = response.message }
            , Cmd.batch 
            [ getInformation
            , delay 0 (AddToast { message = response.message, color = Green } )
            ] 
          )
        
        Err _ ->
          ( model, delay 0 (AddToast { message = "削除中ににエラーが発生しました。", color = Red } ) )

    GotInfos result ->
      case result of
        Ok informations ->
          ( { model | informations = informations }, Cmd.none )

        Err _ ->
          ( model, delay 0 (AddToast { message = "データ取得中ににエラーが発生しました。", color = Red } ) )

    EditInfo information ->
      let 
        editForm =
          model.editForm
      in
      ( { model | editForm = { editForm | url = information.url, tag = information.tag, title = information.title }, selectedInformation = Just information, isPopupOpen = True }, Cmd.none )

    CancelEdit ->
      ( { model | isPopupOpen = False }, Cmd.none )

    AddToast content ->
      let
        ( tray, tmesg ) =
            Toast.add model.tray (Toast.expireIn 3000 content)
      in
      ( { model | tray = tray }, Cmd.map ToastMsg tmesg )

    ToastMsg tmsg ->
      let
        ( tray, newTmesg ) =
            Toast.update tmsg model.tray
      in
      ( { model | tray = tray }, Cmd.map ToastMsg newTmesg )


-- Subscription
subscriptions : Model -> Sub Msg
subscriptions _ =
  Sub.none


-- View
view : Model -> Html Msg
view model =
  div [] 
    [ h2 [ class "heading" ] [ span [] [ text "UOPEOPLE LIFE IS GOING BETTER" ], text "Uoeople まとめ" ]
    , div [ class "card-group" ] 
      ( List.map (viewCard model) model.informations )
    , div [ class "input-block" ]
        [ viewInput "url" "URL" model.form.url UpdateUrl 
        , br [] []
        , viewInput "tag" "Tag" model.form.tag UpdateTag
        , br [] []
        , viewInput "title" "Title" model.form.title UpdateTitle
        , ul [ class "error-messages" ]
          ( List.map viewProblem model.problems )
        ]
    , div [ class "button-block" ] 
        [  button [ onClick Submit ] [ text "Submit"]
        , br [] []
      ]
    , Toast.render viewToast model.tray (Toast.config ToastMsg)
    , if model.isPopupOpen then
        case model.selectedInformation of
          Just selected ->
            div [ class "edit-card" ]
              [ viewInput "url" "Url" model.editForm.url UpdateSelectedUrl
              , br [] []
              , viewInput "title" "Title" model.editForm.tag UpdateSelectedTag
              , br [] []
              , viewInput "title" "Title" model.editForm.title UpdateSelectedTitle
              , br [] []
              , ul [ class "error-messages" ]
                ( List.map viewProblem model.editProblems )
              , button [ onClick (SubmitEdited selected.id) ] [ text "編集" ] 
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


viewProblem : Problem -> Html msg
viewProblem problem =
  let
    errorMessage =
      case problem of
        InvalidEntry _ str ->
            str

        ServerError str ->
            str
  in
  li [] [ text errorMessage ]

viewToast : List (Html.Attribute Msg) -> Toast.Info Toast -> Html Msg
viewToast attributes toast =
  Html.div
    (toastStyles toast ++ attributes)
    [ Html.text toast.content.message ]

toastStyles : Toast.Info Toast -> List (Html.Attribute msg)
toastStyles toast =
  let
    background : Html.Attribute msg
    background =
      case toast.content.color of
        Red ->
          style "background" "#f77"

        Green ->
          style "background" "#7f7"

        Blue ->
          style "background" "#77f"
  in
  [ background, class "toast" ]


-- Cmd
postInforation : TrimmedForm -> Cmd Msg
postInforation (Trimmed form) =
  let 
    json = formJson form
  
  in
  Http.post
    {
      url = "http://localhost:8080/api/v1/info/create"
    , body = Http.jsonBody json
    , expect = Http.expectJson GotCreated responseMsgDecoder
    }

putInforation : Int -> TrimmedForm -> Cmd Msg
putInforation info_id (Trimmed form) =
  let 
    json = editFormmJson info_id form

  in
  Http.post
    {
      url = "http://localhost:8080/api/v1/info/update"
    , body = Http.jsonBody json
    , expect = Http.expectJson GotUpdated responseMsgDecoder
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
    , expect = Http.expectJson GotDeleted responseMsgDecoder
    , timeout = Nothing
    , tracker = Nothing
    }


-- Encoder
deleteJson : Information -> Encode.Value
deleteJson information =
  Encode.object
    [ ( "id", Encode.int information.id )
    ]

-- TODO: 現時点ではユーザー固定
formJson : Form -> Encode.Value
formJson form =
  Encode.object
    [ ( "user_id", Encode.int 1 ) 
    , ( "url", Encode.string form.url )
    , ( "tag", Encode.string form.tag )
    , ( "title", Encode.string form.title )
    ]

editFormmJson : Int -> Form -> Encode.Value
editFormmJson info_id form =
  Encode.object
    [ ( "id", Encode.int info_id ) 
    , ( "url", Encode.string form.url )
    , ( "tag", Encode.string form.tag )
    , ( "title", Encode.string form.title )
    ]


-- Decoder
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

responseMsgDecoder : Decode.Decoder ResponseMessage
responseMsgDecoder =
  Decode.map ResponseMessage
    (Decode.field "message" Decode.string)


-- Varidation
-- 参考: https://github.com/rtfeldman/elm-spa-example/blob/master/src/Page/Login.elm
type TrimmedForm
  = Trimmed Form

type ValidatedField
  = URL
  | Tag
  | Title

fieldsToValidate : List ValidatedField
fieldsToValidate =
  [ URL
  , Tag
  , Title
  ]

validate : Form -> Result (List Problem) TrimmedForm
validate form =
  let
    trimmedForm =
      trimFields form
  in
  case List.concatMap (validateField trimmedForm) fieldsToValidate of
    [] ->
      Ok trimmedForm

    problems ->
      Err problems


validateField : TrimmedForm -> ValidatedField -> List Problem
validateField (Trimmed form) field =
  List.map (InvalidEntry field) <|
    case field of
      URL ->
        if String.isEmpty form.url then
          [ "url can't be blank." ]

        else
            []

      Tag ->
        if String.isEmpty form.tag then
          [ "tag can't be blank." ]

        else
            []

      Title ->
        if String.isEmpty form.title then
          [ "title can't be blank." ]

        else
            []


-- Toast
trimFields : Form -> TrimmedForm
trimFields form =
  Trimmed
    { url = String.trim form.url
    , tag = String.trim form.tag
    , title = String.trim form.title
    }

emptyTray : Toast.Tray Toast
emptyTray =
  Toast.tray

delay : Int -> msg -> Cmd msg
delay ms msg =
  Task.perform (always msg) (Process.sleep <| toFloat ms)
