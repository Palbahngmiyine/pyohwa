module Main exposing (main)

import Browser
import Browser.Events
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
    Sub.batch
        [ Ports.onScroll OnScroll
        , Browser.Events.onKeyDown keyDecoder
        ]


keyDecoder : Decoder Msg
keyDecoder =
    Decode.map3
        (\key ctrlKey metaKey ->
            if (ctrlKey || metaKey) && key == "k" then
                OpenSearch

            else if key == "Escape" then
                OnKeyDown "Escape"

            else
                NoOp
        )
        (Decode.field "key" Decode.string)
        (Decode.field "ctrlKey" Decode.bool)
        (Decode.field "metaKey" Decode.bool)



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
