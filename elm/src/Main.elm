module Main exposing (main)

import Browser
import Flags exposing (PrevNextLink)
import Html exposing (Html)
import Json.Decode as Decode exposing (Decoder, Value)
import Model exposing (Model)
import Msg exposing (Msg(..))
import Ports
import Theme.Layout as Layout
import Update exposing (update)


main : Program Value Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }


init : Value -> ( Model, Cmd Msg )
init flagsValue =
    case Decode.decodeValue flagsWithNavDecoder flagsValue of
        Ok ( flags, prev, next ) ->
            ( Model.fromFlags flags prev next, Cmd.none )

        Err _ ->
            ( Model.fallback, Cmd.none )


view : Model -> Html Msg
view model =
    Layout.view model


subscriptions : Model -> Sub Msg
subscriptions _ =
    Ports.onScroll OnScroll



-- Decode flags along with optional prev/next links


type alias FlagsWithNav =
    ( Flags.Flags, Maybe PrevNextLink, Maybe PrevNextLink )


flagsWithNavDecoder : Decoder FlagsWithNav
flagsWithNavDecoder =
    Decode.map3 (\f p n -> ( f, p, n ))
        Flags.decoder
        (Decode.oneOf
            [ Decode.field "prev" (Decode.map Just Flags.prevNextLinkDecoder)
            , Decode.succeed Nothing
            ]
        )
        (Decode.oneOf
            [ Decode.field "next" (Decode.map Just Flags.prevNextLinkDecoder)
            , Decode.succeed Nothing
            ]
        )
