module Msg exposing (Msg(..))


type Msg
    = ToggleSidebar
    | CloseSidebar
    | ScrollToHeading String
    | OnScroll Float
    | NoOp
