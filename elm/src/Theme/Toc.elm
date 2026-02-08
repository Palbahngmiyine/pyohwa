module Theme.Toc exposing (view)

import Html exposing (Html, a, aside, div, text)
import Html.Attributes exposing (attribute, class, classList, href, id)
import Html.Events exposing (onClick)
import Model exposing (Model)
import Msg exposing (Msg(..))


view : Model -> Html Msg
view model =
    aside [ class "pyohwa-toc", id "toc" ]
        [ div [ class "pyohwa-toc-title" ] [ text "On this page" ]
        , div [] (List.map (viewItem model.activeTocId) model.pageToc)
        ]


viewItem : String -> Model.TocItemModel -> Html Msg
viewItem activeTocId item =
    a
        [ class "pyohwa-toc-link"
        , classList [ ( "active", item.id == activeTocId ) ]
        , attribute "data-level" (String.fromInt item.level)
        , href ("#" ++ item.id)
        , onClick (ScrollToHeading item.id)
        ]
        [ text item.text ]
