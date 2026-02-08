module Theme.Navbar exposing (view)

import Html exposing (Html, a, button, div, nav, span, text)
import Html.Attributes exposing (class, classList, href)
import Html.Events exposing (onClick)
import Model exposing (Model)
import Msg exposing (Msg(..))


view : Model -> Html Msg
view model =
    nav [ class "pyohwa-navbar" ]
        [ div [ class "pyohwa-navbar-inner" ]
            [ hamburgerButton
            , a [ class "pyohwa-navbar-title", href model.siteBase ]
                [ text model.siteTitle ]
            , viewNavLinks model.nav
            , viewSearchButton model.searchEnabled
            ]
        ]


hamburgerButton : Html Msg
hamburgerButton =
    button
        [ class "pyohwa-navbar-hamburger"
        , onClick ToggleSidebar
        ]
        [ span [ class "pyohwa-hamburger-line" ] []
        , span [ class "pyohwa-hamburger-line" ] []
        , span [ class "pyohwa-hamburger-line" ] []
        ]


viewNavLinks : List Model.NavItemModel -> Html Msg
viewNavLinks items =
    div [ class "pyohwa-navbar-links" ]
        (List.map viewNavLink items)


viewNavLink : Model.NavItemModel -> Html Msg
viewNavLink item =
    a
        [ class "pyohwa-navbar-link"
        , classList [ ( "active", item.active ) ]
        , href item.link
        ]
        [ text item.text ]


viewSearchButton : Bool -> Html Msg
viewSearchButton enabled =
    if enabled then
        button
            [ class "pyohwa-navbar-search"
            , onClick OpenSearch
            ]
            [ span [ class "pyohwa-navbar-search-text" ] [ text "Search" ]
            , span [ class "pyohwa-navbar-search-kbd" ] [ text "Ctrl+K" ]
            ]

    else
        text ""
