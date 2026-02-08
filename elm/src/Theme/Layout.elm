module Theme.Layout exposing (view)

import Html exposing (Html, div, main_, node, text)
import Html.Attributes exposing (class, id, property)
import Json.Encode as Encode
import Model exposing (Model)
import Msg exposing (Msg)
import Theme.Footer as Footer
import Theme.Navbar as Navbar
import Theme.Sidebar as Sidebar
import Theme.Toc as Toc


view : Model -> Html Msg
view model =
    div [ class "pyohwa-layout" ]
        [ Navbar.view model
        , viewBody model
        ]


viewBody : Model -> Html Msg
viewBody model =
    case model.pageLayout of
        "home" ->
            viewHomeLayout model

        "page" ->
            viewPageLayout model

        _ ->
            viewDocLayout model


viewDocLayout : Model -> Html Msg
viewDocLayout model =
    div [ class "pyohwa-main" ]
        [ Sidebar.view model
        , main_ [ class "pyohwa-content" ]
            [ div
                [ class "pyohwa-prose"
                , id "content"
                , property "innerHTML" (Encode.string model.pageContent)
                ]
                []
            , Footer.view model
            ]
        , Toc.view model
        ]


viewHomeLayout : Model -> Html Msg
viewHomeLayout model =
    div [ class "pyohwa-layout-home" ]
        [ main_ [ class "pyohwa-content" ]
            [ div
                [ class "pyohwa-prose"
                , id "content"
                , property "innerHTML" (Encode.string model.pageContent)
                ]
                []
            ]
        ]


viewPageLayout : Model -> Html Msg
viewPageLayout model =
    div [ class "pyohwa-layout-page" ]
        [ main_ [ class "pyohwa-content pyohwa-content--centered" ]
            [ div
                [ class "pyohwa-prose"
                , id "content"
                , property "innerHTML" (Encode.string model.pageContent)
                ]
                []
            , Footer.view model
            ]
        ]
