module Model exposing (Model, NavItemModel, SidebarGroupModel, SidebarItemModel, TocItemModel, fallback, fromFlags)

import Flags exposing (Flags, PrevNextLink)
import Search.Search exposing (SearchResult, SearchState(..))


type alias Model =
    { pageTitle : String
    , pageDescription : String
    , pageContent : String
    , pageToc : List TocItemModel
    , pageLayout : String
    , siteTitle : String
    , siteDescription : String
    , siteBase : String
    , nav : List NavItemModel
    , sidebar : List SidebarGroupModel
    , highlightTheme : String
    , prev : Maybe PrevNextLink
    , next : Maybe PrevNextLink
    , sidebarOpen : Bool
    , activeTocId : String
    , searchOpen : Bool
    , searchQuery : String
    , searchResults : List SearchResult
    , searchIndex : SearchState
    , searchEnabled : Bool
    }


type alias TocItemModel =
    { id : String
    , text : String
    , level : Int
    }


type alias NavItemModel =
    { text : String
    , link : String
    , active : Bool
    }


type alias SidebarGroupModel =
    { text : String
    , items : List SidebarItemModel
    }


type alias SidebarItemModel =
    { text : String
    , link : String
    , active : Bool
    }


fromFlags : Flags -> Maybe PrevNextLink -> Maybe PrevNextLink -> Model
fromFlags flags prev next =
    { pageTitle = flags.page.title
    , pageDescription = flags.page.description
    , pageContent = flags.page.content
    , pageToc =
        List.map
            (\item ->
                { id = item.id
                , text = item.text
                , level = item.level
                }
            )
            flags.page.toc
    , pageLayout = flags.page.layout
    , siteTitle = flags.site.title
    , siteDescription = flags.site.description
    , siteBase = flags.site.base
    , nav =
        List.map
            (\item ->
                { text = item.text
                , link = item.link
                , active = item.active
                }
            )
            flags.site.nav
    , sidebar =
        List.map
            (\group ->
                { text = group.text
                , items =
                    List.map
                        (\item ->
                            { text = item.text
                            , link = item.link
                            , active = item.active
                            }
                        )
                        group.items
                }
            )
            flags.site.sidebar
    , highlightTheme = flags.theme.highlightTheme
    , prev = prev
    , next = next
    , sidebarOpen = False
    , activeTocId = ""
    , searchOpen = False
    , searchQuery = ""
    , searchResults = []
    , searchIndex = Idle
    , searchEnabled = flags.search.enabled
    }


fallback : Model
fallback =
    { pageTitle = "Error"
    , pageDescription = ""
    , pageContent = "<p>Failed to load page data.</p>"
    , pageToc = []
    , pageLayout = "doc"
    , siteTitle = "Pyohwa"
    , siteDescription = ""
    , siteBase = "/"
    , nav = []
    , sidebar = []
    , highlightTheme = "one-dark"
    , prev = Nothing
    , next = Nothing
    , sidebarOpen = False
    , activeTocId = ""
    , searchOpen = False
    , searchQuery = ""
    , searchResults = []
    , searchIndex = Idle
    , searchEnabled = True
    }
