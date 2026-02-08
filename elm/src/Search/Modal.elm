module Search.Modal exposing (view)

import Html exposing (Html, a, div, input, p, span, text)
import Html.Attributes exposing (class, href, placeholder, type_, value)
import Html.Events exposing (onClick, onInput, stopPropagationOn)
import Json.Decode as Decode
import Model exposing (Model)
import Msg exposing (Msg(..))
import Search.Search exposing (SearchResult)


view : Model -> Html Msg
view model =
    if not model.searchOpen then
        text ""

    else
        div
            [ class "pyohwa-search-overlay"
            , onClick CloseSearch
            ]
            [ div
                [ class "pyohwa-search-modal"
                , stopPropagationOn "click" (Decode.succeed ( NoOp, True ))
                ]
                [ viewInput model.searchQuery
                , viewResults model.searchResults
                , viewFooter
                ]
            ]


viewInput : String -> Html Msg
viewInput query =
    input
        [ class "pyohwa-search-input"
        , type_ "text"
        , placeholder "Search documentation..."
        , value query
        , onInput SearchInput
        ]
        []


viewResults : List SearchResult -> Html Msg
viewResults results =
    if List.isEmpty results then
        div [ class "pyohwa-search-results pyohwa-search-empty" ]
            [ p [ class "pyohwa-search-hint" ] [ text "Type to search..." ] ]

    else
        div [ class "pyohwa-search-results" ]
            (List.map viewResult results)


viewResult : SearchResult -> Html Msg
viewResult result =
    a
        [ class "pyohwa-search-result"
        , href result.url
        ]
        [ div [ class "pyohwa-search-result-title" ] [ text result.title ]
        , div [ class "pyohwa-search-result-desc" ] [ text result.description ]
        , div [ class "pyohwa-search-result-context" ] [ text result.matchContext ]
        ]


viewFooter : Html Msg
viewFooter =
    div [ class "pyohwa-search-footer" ]
        [ span [] [ text "ESC to close" ]
        , span [] [ text "Enter to select" ]
        ]
