module Theme.Sidebar exposing (view)

import Html exposing (Html, a, aside, div, text)
import Html.Attributes exposing (class, classList, href, id)
import Html.Events exposing (onClick)
import Model exposing (Model)
import Msg exposing (Msg(..))


view : Model -> Html Msg
view model =
    div []
        [ aside
            [ class "pyohwa-sidebar"
            , classList [ ( "pyohwa-sidebar--open", model.sidebarOpen ) ]
            , id "sidebar"
            ]
            (List.map viewGroup model.sidebar)
        , overlay model.sidebarOpen
        ]


overlay : Bool -> Html Msg
overlay isOpen =
    if isOpen then
        div
            [ class "pyohwa-sidebar-overlay"
            , onClick CloseSidebar
            ]
            []

    else
        text ""


viewGroup : Model.SidebarGroupModel -> Html Msg
viewGroup group =
    div [ class "pyohwa-sidebar-group" ]
        [ div [ class "pyohwa-sidebar-group-title" ]
            [ text group.text ]
        , div [] (List.map viewItem group.items)
        ]


viewItem : Model.SidebarItemModel -> Html Msg
viewItem item =
    a
        [ class "pyohwa-sidebar-link"
        , classList [ ( "active", item.active ) ]
        , href item.link
        ]
        [ text item.text ]
