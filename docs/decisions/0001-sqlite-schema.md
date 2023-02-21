# Schema Design

## Context and Problem Statement

We need a defined schema for our application that balances normalization without
getting too tunnel-visioned on efficiency.
It should represent Items filled with metadata and local user ratings to start.

## Design Options

One option is going with `ITEM`'s that can have multiple `CATEGORY`'s. If we choose
to store ratings as a simple number and doing the parsing later,
that saves us another data structure.

We'll have an `items`, `categories` and `ratings` table - with two joining tables that link
`items` to `ratings` and `items` to `categories`.

```mermaid
erDiagram
    items {
        integer   item_id
        varchar   name
        varchar   description
        varchar   comments
    }
    categories {
        integer   category_id
        varchar   name
        varchar   description
    }
    ratings {
        integer   rating_id
        integer   rating
        date   date
    }
    items_to_ratings {
        integer   item_id
        integer   rating_id
    }
    items_to_categories {
        integer   item_id
        integer   category_id
    }

     items ||--|{ items_to_ratings : have
     ratings ||--|{ items_to_ratings : have
     categories ||--|{ items_to_categories : have
     items ||--|{ items_to_categories : have
```
