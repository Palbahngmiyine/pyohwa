module Update exposing (update)

import Model exposing (Model)
import Msg exposing (Msg(..))
import Ports


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

        NoOp ->
            ( model, Cmd.none )


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
