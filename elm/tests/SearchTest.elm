module SearchTest exposing (..)

import Expect
import Search.Search exposing (SearchEntry, filterResults)
import Test exposing (..)


sampleEntries : List SearchEntry
sampleEntries =
    [ { id = "1"
      , url = "/guide/intro"
      , title = "Introduction"
      , description = "Getting started guide"
      , content = "Welcome to the documentation. This guide helps you get started."
      , tags = [ "guide", "intro" ]
      }
    , { id = "2"
      , url = "/api/config"
      , title = "Configuration"
      , description = "How to configure the system"
      , content = "You can configure pyohwa using a toml file."
      , tags = [ "api", "config" ]
      }
    , { id = "3"
      , url = "/guide/advanced"
      , title = "Advanced Usage"
      , description = "Advanced features"
      , content = "This section covers advanced topics like custom themes."
      , tags = [ "guide", "advanced" ]
      }
    ]


suite : Test
suite =
    describe "Search.filterResults"
        [ test "short query returns empty" <|
            \_ ->
                Expect.equal [] (filterResults "a" sampleEntries)
        , test "matches by title" <|
            \_ ->
                let
                    results =
                        filterResults "Introduction" sampleEntries
                in
                Expect.equal 1 (List.length results)
        , test "matches by content" <|
            \_ ->
                let
                    results =
                        filterResults "toml" sampleEntries
                in
                Expect.equal 1 (List.length results)
        , test "matches by tags" <|
            \_ ->
                let
                    results =
                        filterResults "advanced" sampleEntries
                in
                Expect.atLeast 1 (List.length results)
        , test "case insensitive matching" <|
            \_ ->
                let
                    results =
                        filterResults "CONFIGURATION" sampleEntries
                in
                Expect.equal 1 (List.length results)
        , test "limits to 10 results" <|
            \_ ->
                let
                    manyEntries =
                        List.repeat 20
                            { id = "x"
                            , url = "/test"
                            , title = "Test Page"
                            , description = "test"
                            , content = "matching content"
                            , tags = []
                            }

                    results =
                        filterResults "test" manyEntries
                in
                Expect.atMost 10 (List.length results)
        ]
