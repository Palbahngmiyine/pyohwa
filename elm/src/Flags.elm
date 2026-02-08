module Flags exposing (Flags, PrevNextLink, SearchData, decoder, prevNextLinkDecoder)

import Json.Decode as Decode exposing (Decoder)


type alias Flags =
    { page : PageData
    , site : SiteData
    , theme : ThemeData
    , search : SearchData
    }


type alias PageData =
    { title : String
    , description : String
    , content : String
    , toc : List TocItem
    , layout : String
    }


type alias TocItem =
    { id : String
    , text : String
    , level : Int
    }


type alias SiteData =
    { title : String
    , description : String
    , base : String
    , nav : List NavItem
    , sidebar : List SidebarGroup
    }


type alias NavItem =
    { text : String
    , link : String
    , active : Bool
    }


type alias SidebarGroup =
    { text : String
    , items : List SidebarItem
    }


type alias SidebarItem =
    { text : String
    , link : String
    , active : Bool
    }


type alias ThemeData =
    { highlightTheme : String
    }


type alias SearchData =
    { enabled : Bool
    }


type alias PrevNextLink =
    { title : String
    , link : String
    }



-- DECODERS


decoder : Decoder Flags
decoder =
    Decode.map4 Flags
        (Decode.field "page" pageDecoder)
        (Decode.field "site" siteDecoder)
        (Decode.field "theme" themeDecoder)
        (Decode.oneOf
            [ Decode.field "search" searchDecoder
            , Decode.succeed { enabled = True }
            ]
        )


pageDecoder : Decoder PageData
pageDecoder =
    Decode.map5 PageData
        (Decode.field "title" Decode.string)
        (Decode.field "description" Decode.string)
        (Decode.field "content" Decode.string)
        (Decode.field "toc" (Decode.list tocItemDecoder))
        (Decode.oneOf
            [ Decode.field "layout" Decode.string
            , Decode.succeed "doc"
            ]
        )


tocItemDecoder : Decoder TocItem
tocItemDecoder =
    Decode.map3 TocItem
        (Decode.field "id" Decode.string)
        (Decode.field "text" Decode.string)
        (Decode.field "level" Decode.int)


siteDecoder : Decoder SiteData
siteDecoder =
    Decode.map5 SiteData
        (Decode.field "title" Decode.string)
        (Decode.field "description" Decode.string)
        (Decode.field "base" Decode.string)
        (Decode.field "nav" (Decode.list navItemDecoder))
        (Decode.field "sidebar" (Decode.list sidebarGroupDecoder))


navItemDecoder : Decoder NavItem
navItemDecoder =
    Decode.map3 NavItem
        (Decode.field "text" Decode.string)
        (Decode.field "link" Decode.string)
        (Decode.oneOf
            [ Decode.field "active" Decode.bool
            , Decode.succeed False
            ]
        )


sidebarGroupDecoder : Decoder SidebarGroup
sidebarGroupDecoder =
    Decode.map2 SidebarGroup
        (Decode.field "text" Decode.string)
        (Decode.field "items" (Decode.list sidebarItemDecoder))


sidebarItemDecoder : Decoder SidebarItem
sidebarItemDecoder =
    Decode.map3 SidebarItem
        (Decode.field "text" Decode.string)
        (Decode.field "link" Decode.string)
        (Decode.oneOf
            [ Decode.field "active" Decode.bool
            , Decode.succeed False
            ]
        )


themeDecoder : Decoder ThemeData
themeDecoder =
    Decode.map ThemeData
        (Decode.field "highlightTheme" Decode.string)


searchDecoder : Decoder SearchData
searchDecoder =
    Decode.map SearchData
        (Decode.field "enabled" Decode.bool)


prevNextLinkDecoder : Decoder PrevNextLink
prevNextLinkDecoder =
    Decode.map2 PrevNextLink
        (Decode.field "title" Decode.string)
        (Decode.field "link" Decode.string)
