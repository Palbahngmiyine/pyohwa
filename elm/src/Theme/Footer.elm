module Theme.Footer exposing (view)

import Flags exposing (PrevNextLink)
import Html exposing (Html, a, div, footer, span, text)
import Html.Attributes exposing (class, href)
import Model exposing (Model)
import Msg exposing (Msg)


view : Model -> Html Msg
view model =
    footer [ class "pyohwa-footer" ]
        [ div [ class "pyohwa-footer-nav" ]
            [ viewPrev model.prev
            , viewNext model.next
            ]
        ]


viewPrev : Maybe PrevNextLink -> Html Msg
viewPrev maybePrev =
    case maybePrev of
        Just link ->
            a [ class "pyohwa-footer-link pyohwa-footer-prev", href link.link ]
                [ span [ class "pyohwa-footer-label" ] [ text "Previous" ]
                , span [ class "pyohwa-footer-title" ] [ text link.title ]
                ]

        Nothing ->
            div [] []


viewNext : Maybe PrevNextLink -> Html Msg
viewNext maybeNext =
    case maybeNext of
        Just link ->
            a [ class "pyohwa-footer-link pyohwa-footer-next", href link.link ]
                [ span [ class "pyohwa-footer-label" ] [ text "Next" ]
                , span [ class "pyohwa-footer-title" ] [ text link.title ]
                ]

        Nothing ->
            div [] []
