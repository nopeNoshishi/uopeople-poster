module Main exposing (Model, Msg(..), init, main, update, view)

import Browser
import Html exposing (Html, div, h1, img, text)
import Html.Attributes exposing (src)
import Http
import Json.Decode as Decode
import Url.Builder as Url





type alias Model =
 { message : String }


init : () -> ( Model, Cmd Msg )
init _ =
 ( { message = "" }, getHealthCheck )





type Msg
 = HealthCheck (Result Http.Error String)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
 case msg of
  HealthCheck (Ok res) ->
    ( { model | message = res }, Cmd.none )

  HealthCheck (Err _) ->
    ( { model | message = "error" }, Cmd.none )





view : Model -> Html Msg
view model =
 div []
 [ h1 [] [ text model.message ]
 ]


subscriptions : Model -> Sub Msg
subscriptions model =
 Sub.none


main =
 Browser.element
 { init = init
 , update = update
 , subscriptions = subscriptions
 , view = view
 }


getHealthCheck : Cmd Msg
getHealthCheck =
 Http.send HealthCheck (Http.post getUrl decoder)


getUrl : String
getUrl =
 Url.crossOrigin "http://localhost:8080" [ "api/v1/health" ] []


decoder : Decode.Decoder String
decoder =
 Decode.field "message" Decode.string