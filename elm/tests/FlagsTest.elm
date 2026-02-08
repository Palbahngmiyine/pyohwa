module FlagsTest exposing (..)

import Expect
import Flags
import Json.Decode as Decode
import Test exposing (..)


suite : Test
suite =
    describe "Flags decoder"
        [ test "decodes valid minimal JSON" <|
            \_ ->
                let
                    json =
                        """
                        {
                            "page": {
                                "title": "Hello",
                                "description": "A page",
                                "content": "<p>Hi</p>",
                                "toc": [],
                                "layout": "doc",
                                "frontmatter": {}
                            },
                            "site": {
                                "title": "My Site",
                                "description": "Desc",
                                "base": "/",
                                "nav": [],
                                "sidebar": []
                            },
                            "theme": {
                                "highlightTheme": "one-dark"
                            }
                        }
                        """
                in
                case Decode.decodeString Flags.decoder json of
                    Ok flags ->
                        Expect.all
                            [ \f -> Expect.equal "Hello" f.page.title
                            , \f -> Expect.equal "My Site" f.site.title
                            , \f -> Expect.equal "one-dark" f.theme.highlightTheme
                            , \f -> Expect.equal "doc" f.page.layout
                            ]
                            flags

                    Err err ->
                        Expect.fail (Decode.errorToString err)
        , test "decodes full JSON with nav and sidebar" <|
            \_ ->
                let
                    json =
                        """
                        {
                            "page": {
                                "title": "Guide",
                                "description": "",
                                "content": "<h1>Guide</h1>",
                                "toc": [{"id": "guide", "text": "Guide", "level": 1}],
                                "layout": "doc",
                                "frontmatter": {}
                            },
                            "site": {
                                "title": "Docs",
                                "description": "Documentation",
                                "base": "/docs/",
                                "nav": [{"text": "Home", "link": "/", "active": true}],
                                "sidebar": [{"text": "Getting Started", "items": [{"text": "Intro", "link": "/intro", "active": true}]}]
                            },
                            "theme": {
                                "highlightTheme": "monokai"
                            }
                        }
                        """
                in
                case Decode.decodeString Flags.decoder json of
                    Ok flags ->
                        Expect.all
                            [ \f -> Expect.equal 1 (List.length f.page.toc)
                            , \f -> Expect.equal 1 (List.length f.site.nav)
                            , \f -> Expect.equal 1 (List.length f.site.sidebar)
                            ]
                            flags

                    Err err ->
                        Expect.fail (Decode.errorToString err)
        , test "fails on missing page field" <|
            \_ ->
                let
                    json =
                        """
                        {
                            "site": {"title": "X", "description": "", "base": "/", "nav": [], "sidebar": []},
                            "theme": {"highlightTheme": "one-dark"}
                        }
                        """
                in
                case Decode.decodeString Flags.decoder json of
                    Ok _ ->
                        Expect.fail "Should have failed"

                    Err _ ->
                        Expect.pass
        , test "defaults layout to doc when missing" <|
            \_ ->
                let
                    json =
                        """
                        {
                            "page": {
                                "title": "No Layout",
                                "description": "",
                                "content": "",
                                "toc": [],
                                "frontmatter": {}
                            },
                            "site": {"title": "S", "description": "", "base": "/", "nav": [], "sidebar": []},
                            "theme": {"highlightTheme": "x"}
                        }
                        """
                in
                case Decode.decodeString Flags.decoder json of
                    Ok flags ->
                        Expect.equal "doc" flags.page.layout

                    Err err ->
                        Expect.fail (Decode.errorToString err)
        , test "decodes prevNextLink" <|
            \_ ->
                let
                    json =
                        """{"title": "Next Page", "link": "/next"}"""
                in
                case Decode.decodeString Flags.prevNextLinkDecoder json of
                    Ok link ->
                        Expect.all
                            [ \l -> Expect.equal "Next Page" l.title
                            , \l -> Expect.equal "/next" l.link
                            ]
                            link

                    Err err ->
                        Expect.fail (Decode.errorToString err)
        , test "decodes search enabled setting" <|
            \_ ->
                let
                    json =
                        """
                        {
                            "page": {"title": "T", "description": "", "content": "", "toc": [], "layout": "doc", "frontmatter": {}},
                            "site": {"title": "S", "description": "", "base": "/", "nav": [], "sidebar": []},
                            "theme": {"highlightTheme": "x"},
                            "search": {"enabled": false}
                        }
                        """
                in
                case Decode.decodeString Flags.decoder json of
                    Ok flags ->
                        Expect.equal False flags.search.enabled

                    Err err ->
                        Expect.fail (Decode.errorToString err)
        , test "search defaults to enabled when missing" <|
            \_ ->
                let
                    json =
                        """
                        {
                            "page": {"title": "T", "description": "", "content": "", "toc": [], "layout": "doc", "frontmatter": {}},
                            "site": {"title": "S", "description": "", "base": "/", "nav": [], "sidebar": []},
                            "theme": {"highlightTheme": "x"}
                        }
                        """
                in
                case Decode.decodeString Flags.decoder json of
                    Ok flags ->
                        Expect.equal True flags.search.enabled

                    Err err ->
                        Expect.fail (Decode.errorToString err)
        ]
