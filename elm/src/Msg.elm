module Msg exposing (Msg(..))

import Http
import Search.Search exposing (SearchEntry)


type Msg
    = ToggleSidebar
    | CloseSidebar
    | ScrollToHeading String
    | OnScroll Float
    | OpenSearch
    | CloseSearch
    | SearchInput String
    | GotSearchIndex (Result Http.Error (List SearchEntry))
    | OnKeyDown String
    | NoOp
