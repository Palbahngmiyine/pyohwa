port module Ports exposing (onScroll, scrollToElement)


port scrollToElement : String -> Cmd msg


port onScroll : (Float -> msg) -> Sub msg
