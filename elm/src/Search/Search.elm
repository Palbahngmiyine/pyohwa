module Search.Search exposing (SearchEntry, SearchResult, SearchState(..), extractContext, filterResults, searchEntryDecoder, searchIndexDecoder)

import Json.Decode as Decode exposing (Decoder)


type SearchState
    = Idle
    | Loading
    | Loaded (List SearchEntry)
    | Error String


type alias SearchEntry =
    { id : String
    , url : String
    , title : String
    , description : String
    , content : String
    , tags : List String
    }


type alias SearchResult =
    { url : String
    , title : String
    , description : String
    , matchContext : String
    }


filterResults : String -> List SearchEntry -> List SearchResult
filterResults query entries =
    if String.length query < 2 then
        []

    else
        let
            lowerQuery =
                String.toLower query
        in
        entries
            |> List.filterMap (matchEntry lowerQuery)
            |> List.take 10


matchEntry : String -> SearchEntry -> Maybe SearchResult
matchEntry lowerQuery entry =
    let
        lowerTitle =
            String.toLower entry.title

        lowerDesc =
            String.toLower entry.description

        lowerContent =
            String.toLower entry.content

        lowerTags =
            List.map String.toLower entry.tags

        titleMatch =
            String.contains lowerQuery lowerTitle

        descMatch =
            String.contains lowerQuery lowerDesc

        contentMatch =
            String.contains lowerQuery lowerContent

        tagMatch =
            List.any (String.contains lowerQuery) lowerTags
    in
    if titleMatch || descMatch || contentMatch || tagMatch then
        Just
            { url = entry.url
            , title = entry.title
            , description = entry.description
            , matchContext = extractContext lowerQuery entry.content
            }

    else
        Nothing


extractContext : String -> String -> String
extractContext query content =
    let
        lowerContent =
            String.toLower content

        lowerQuery =
            String.toLower query
    in
    case findIndex lowerQuery lowerContent of
        Nothing ->
            String.left 80 content

        Just idx ->
            let
                start =
                    max 0 (idx - 40)

                end =
                    min (String.length content) (idx + String.length query + 40)

                snippet =
                    String.slice start end content

                prefix =
                    if start > 0 then
                        "..."

                    else
                        ""

                suffix =
                    if end < String.length content then
                        "..."

                    else
                        ""
            in
            prefix ++ snippet ++ suffix


findIndex : String -> String -> Maybe Int
findIndex needle haystack =
    findIndexHelper needle haystack 0


findIndexHelper : String -> String -> Int -> Maybe Int
findIndexHelper needle haystack offset =
    if String.length haystack - offset < String.length needle then
        Nothing

    else if String.startsWith needle (String.dropLeft offset haystack) then
        Just offset

    else
        findIndexHelper needle haystack (offset + 1)



-- DECODERS


searchEntryDecoder : Decoder SearchEntry
searchEntryDecoder =
    Decode.map6 SearchEntry
        (Decode.field "id" Decode.string)
        (Decode.field "url" Decode.string)
        (Decode.field "title" Decode.string)
        (Decode.field "description" Decode.string)
        (Decode.field "content" Decode.string)
        (Decode.field "tags" (Decode.list Decode.string))


searchIndexDecoder : Decoder (List SearchEntry)
searchIndexDecoder =
    Decode.field "pages" (Decode.list searchEntryDecoder)
