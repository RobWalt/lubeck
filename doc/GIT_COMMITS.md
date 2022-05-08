# Format

The git commits should have the following format:

```
<type>(<scope>): <subject>      | this is mandatory
<NEWLINE><body>                 | this is optional
<NEWLINE><footer>               | this is optional
```

## `<type>` section

Currently, the following values can be used for `<type>`:

### Work on the library
- `feat` - new features in the library
- `fix` - bugfixes
- `test` - anything regarding tests

### Work on the book
- `book` - work on the book

### Work on the repo
- `doc` - documentation
- `chore` - CI, reorganizing the repo etc.

### Example

```
book(...): ...
```

## `<scope>` section

The value for `<scope>` can be anything summarizing the topic we are working on in one word

### Example

```
book(introduction): ...
```

## `<subject>` section

A short description what we changed with our commit should replace
`<subject>`. Anything over ten words should be split up into smaller chunks and
described in the `<body>` section instead.

Additionally we:
- use imperative, present tense: "change" instead of "changed" or "changes"
- don't capitalize the first letter of anything included in the subject 
- don't use dots `.` at the end of the subject

### Example

```
book(introduction): add link to new chapter
```

## `<body>` section 

This section is optional and can be used for detailed descriptions of our commit.
There are no restrictions to the contents of the body.

### Example

```
book(introduction): add link to new chapter

Nobody seemed to find our new cool chapter until we found out that it is not
discoverable besides guessing its URL. That's why we added another link on the
introduction page.
```

## `<footer>` section 

This section is optional and can be used to draw attention to breaking changes
as well as references to related issues.

### Example

```
book(introduction): add link to new chapter

Nobody seemed to find our new cool chapter until we found out that it is not
discoverable besides guessing its URL. That's why we added another link on the
introduction page.

relates to #148, closes #841
```
