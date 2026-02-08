module Update exposing (update)

import Http
import Model exposing (Model)
import Msg exposing (Msg(..))
import Ports
import Search.Search as Search exposing (SearchState(..))


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ToggleSidebar ->
            ( { model | sidebarOpen = not model.sidebarOpen }, Cmd.none )

        CloseSidebar ->
            ( { model | sidebarOpen = False }, Cmd.none )

        ScrollToHeading id ->
            ( { model | activeTocId = id }, Ports.scrollToElement id )

        OnScroll scrollY ->
            ( { model | activeTocId = findActiveHeading scrollY model.pageToc }, Cmd.none )

        OpenSearch ->
            case model.searchIndex of
                Idle ->
                    ( { model | searchOpen = True, searchIndex = Loading }
                    , fetchSearchIndex model.siteBase
                    )

                _ ->
                    ( { model | searchOpen = True }, Cmd.none )

        CloseSearch ->
            ( { model | searchOpen = False, searchQuery = "", searchResults = [] }, Cmd.none )

        SearchInput query ->
            let
                results =
                    case model.searchIndex of
                        Loaded entries ->
                            Search.filterResults query entries

                        _ ->
                            []
            in
            ( { model | searchQuery = query, searchResults = results }, Cmd.none )

        GotSearchIndex result ->
            case result of
                Ok entries ->
                    let
                        results =
                            Search.filterResults model.searchQuery entries
                    in
                    ( { model | searchIndex = Loaded entries, searchResults = results }, Cmd.none )

                Err _ ->
                    ( { model | searchIndex = Error "Failed to load search index" }, Cmd.none )

        OnKeyDown key ->
            if key == "Escape" then
                ( { model | searchOpen = False, searchQuery = "", searchResults = [] }, Cmd.none )

            else
                ( model, Cmd.none )

        NoOp ->
            ( model, Cmd.none )


fetchSearchIndex : String -> Cmd Msg
fetchSearchIndex base =
    let
        url =
            if String.endsWith "/" base then
                base ++ "search-index.json"

            else
                base ++ "/search-index.json"
    in
    Http.get
        { url = url
        , expect = Http.expectJson GotSearchIndex Search.searchIndexDecoder
        }


findActiveHeading : Float -> List Model.TocItemModel -> String
findActiveHeading scrollY tocItems =
    let
        offset =
            100

        candidates =
            List.filter (\_ -> True) tocItems
    in
    candidates
        |> List.reverse
        |> List.head
        |> Maybe.map .id
        |> Maybe.withDefault
            (tocItems
                |> List.head
                |> Maybe.map .id
                |> Maybe.withDefault ""
            )
