module UpdateTest exposing (..)

import Expect
import Model
import Msg exposing (Msg(..))
import Test exposing (..)
import Update


fallbackModel : Model.Model
fallbackModel =
    Model.fallback


suite : Test
suite =
    describe "Update"
        [ test "ToggleSidebar opens closed sidebar" <|
            \_ ->
                let
                    model =
                        { fallbackModel | sidebarOpen = False }

                    ( newModel, _ ) =
                        Update.update ToggleSidebar model
                in
                Expect.equal True newModel.sidebarOpen
        , test "ToggleSidebar closes open sidebar" <|
            \_ ->
                let
                    model =
                        { fallbackModel | sidebarOpen = True }

                    ( newModel, _ ) =
                        Update.update ToggleSidebar model
                in
                Expect.equal False newModel.sidebarOpen
        , test "CloseSidebar always closes" <|
            \_ ->
                let
                    model =
                        { fallbackModel | sidebarOpen = True }

                    ( newModel, _ ) =
                        Update.update CloseSidebar model
                in
                Expect.equal False newModel.sidebarOpen
        , test "ScrollToHeading sets activeTocId" <|
            \_ ->
                let
                    ( newModel, _ ) =
                        Update.update (ScrollToHeading "section-2") fallbackModel
                in
                Expect.equal "section-2" newModel.activeTocId
        , test "NoOp does nothing" <|
            \_ ->
                let
                    ( newModel, _ ) =
                        Update.update NoOp fallbackModel
                in
                Expect.equal fallbackModel newModel
        ]
